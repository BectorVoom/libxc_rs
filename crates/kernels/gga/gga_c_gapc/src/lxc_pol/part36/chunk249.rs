//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 249/1328 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk249<F: Float>(t213: F, t218: F, t62: F, t689: F, t215: F, t220: F, t43: F, t126: F, t173: F, zeta_threshold: F) -> (F, F, F, F) {
    let t214 = t213 <= zeta_threshold;
    let t219 = t218 <= zeta_threshold;
    let t978 = -t62 - t689;
    let t981 = piecewise3::<f64>(t214, F::new(0.0), F::new(4.0) / F::new(3.0) * t215 * t978);
    let t982 = -t978;
    let t985 = piecewise3::<f64>(t219, F::new(0.0), F::new(4.0) / F::new(3.0) * t220 * t982);
    let t987 = (t981 + t985) * t43;
    let t991 = t126 * t173;
    (t978, t982, t987, t991)
}

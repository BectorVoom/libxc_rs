//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 549/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk549<F: Float>(t51: F, t3010: F, t476: F, t3008: F, t3006: F, zeta_threshold: F) -> (F, F) {
    let t52 = t51 <= zeta_threshold;
    let t3011 = t476 * t3010;
    let t3014 = piecewise3::<f64>(t52, F::new(0.0), -F::new(2.0) / F::new(9.0) * t3008 + F::new(2.0) / F::new(3.0) * t3011);
    let t3016 = t3006 / F::new(2.0) + t3014 / F::new(2.0);
    (t3011, t3016)
}

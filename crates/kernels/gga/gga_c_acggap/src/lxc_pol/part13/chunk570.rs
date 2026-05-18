//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 570/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk570<F: Float>(t43: F, t1281: F, t1284: F, t292: F, t39: F, t4000: F, t4070: F, t4073: F, t818: F, t821: F, t824: F, t2910: F, t478: F, zeta_threshold: F) -> (F, F) {
    let t44 = t43 <= zeta_threshold;
    let t4083 = piecewise3::<f64>(t44, F::new(0.0), F::new(8.0) / F::new(27.0) * t4070 * t818 - F::new(8.0) / F::new(9.0) * t4073 * t4000 - F::new(2.0) / F::new(9.0) * t1281 * t824 + F::new(4.0) / F::new(3.0) * t292 * t821 - F::new(4.0) * t1284 * t39);
    let t4084 = t2910 * t478;
    (t4083, t4084)
}

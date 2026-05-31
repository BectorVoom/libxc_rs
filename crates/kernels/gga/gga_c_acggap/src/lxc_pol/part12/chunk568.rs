//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 568/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk568<F: Float>(t43: F, t1281: F, t1284: F, t292: F, t39: F, t4000: F, t4070: F, t4073: F, t818: F, t821: F, t824: F, t2910: F, t478: F, zeta_threshold: F) -> (F, F) {
    let t44 = t43 <= zeta_threshold;
    let t4083 = piecewise3::<F>(t44, F::cast_from(0.0_f64), F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t4070 * t818 - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t4073 * t4000 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1281 * t824 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t292 * t821 - F::cast_from(4.0_f64) * t1284 * t39);
    let t4084 = t2910 * t478;
    (t4083, t4084)
}

//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3675/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3675<F: Float>(t3451: F, t6481: F, t1188: F, t12423: F, t12470: F, t12486: F, t12511: F, t16966: F, t17085: F, t1745: F, t20606: F, t20609: F, t20612: F, t20671: F, t3452: F, t3453: F, t3454: F, t3471: F, t3477: F, t3479: F, t3496: F, t3497: F, t3515: F, t45197: F, t58005: F, t6487: F, t6506: F, t6535: F, t68795: F, t69094: F, t69097: F, t69099: F, t69101: F, t69103: F, t69105: F, t69107: F, t69367: F) -> F {
    let t69488 = t6481 * t3451;
    let t69500 = F::cast_from(0.4138081033541872024e4_f64) * t58005 * t16966 + F::new(12.0) * t12423 * t20606 + F::new(6.0) * t3477 * t6487 * t3471 + F::cast_from(0.11579025239058625248e4_f64) * t12470 * t6506 * t3453 - F::new(8.0) * t12511 * t20609 - F::new(4.0) * t3452 * t1745 * t17085 - F::cast_from(0.38596750796862084162e3_f64) * t45197 * t20612 - t69094 + F::cast_from(0.64327917994770140268e2_f64) * t3477 * t69367 * t3479 - F::new(2.0) * t69488 * t3454 + t69097 - F::cast_from(0.23392894490538584828e1_f64) * t3496 * t68795 * t1188 - t69099 - t69101 + t69103 - t69105 - t69107 - F::cast_from(0.11696447245269292414e1_f64) * t3496 * t6535 * t3515 - F::cast_from(0.10389515463408878255e3_f64) * t12486 * t20671 * t3497;
    t69500
}

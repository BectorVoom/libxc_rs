//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1029/1468 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1029(t12311: f64, t734: f64, t10645: f64, t10647: f64, t10685: f64, t12291: f64, t12294: f64, t12297: f64, t12306: f64, t1841: f64, t1897: f64, t2508: f64, t270: f64, t3723: f64, t3727: f64, t650: f64, t681: f64, t9635: f64, t9651: f64, t9654: f64) -> (f64, f64) {
    let t12312 = t12311 * t734;
    let t12315 = -t10645 - t10647 + 0.76905262301422242837e-2_f64 * t2508 * t12291 + 0.15381052460284448567e-1_f64 * t2508 * t12294 + 0.76905262301422242837e-2_f64 * t1897 * t12297 - t9635 - t9651 + t9654 + 0.10254034973522965712e-1_f64 * t650 * t3723 + 0.76905262301422242837e-2_f64 * t681 * t3723 + 0.76905262301422242837e-2_f64 * t270 * t12306 - 0.10254034973522965712e-1_f64 * t650 * t3727 + t10685 - 0.85450291446024714263e-3_f64 * t1841 * t12312;
    (t12312, t12315)
}

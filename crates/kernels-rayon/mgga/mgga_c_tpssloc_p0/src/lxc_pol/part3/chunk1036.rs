//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 3 (v3rho3_1) CSE chunk 1036/1255 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part3_v3rho3_1_chunk1036(t13048: f64, t13470: f64, t12910: f64, t12914: f64, t12915: f64, t12922: f64, t12926: f64, t12927: f64, t12928: f64, t12934: f64, t12935: f64, t12942: f64, t12944: f64, t12947: f64, t12971: f64, t1484: f64, t1877: f64, t193: f64, t202: f64, t2522: f64, t2523: f64, t2745: f64, t2749: f64, t4255: f64, t4307: f64, t4314: f64, t766: f64, t870: f64, t9470: f64, t9724: f64, t9780: f64, t9863: f64) -> f64 {
    let t13471 = t13048 + t13470;
    let t13475 = t12910 + t9724 + 12.0_f64 * t4314 * t2523 * t4255 + t12914 + t9863 + t9780 + 2.0_f64 * t1877 * t12915 * t2749 - t1877 * t4307 * t2745 + t12922 + t12926 + t12927 - t12928 - 3.0_f64 * t2522 * t9470 * t1484 + t12934 + 6.0_f64 * t193 * t12935 * t1484 + t12942 + t12944 + t12947 + 3.0_f64 * t193 * t766 * t12971 + t193 * t202 * t13471 * t870;
    t13475
}

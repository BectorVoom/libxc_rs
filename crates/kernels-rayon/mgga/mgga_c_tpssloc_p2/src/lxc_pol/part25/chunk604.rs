//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 604/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk604(t1291: f64, t2663: f64, t1284: f64, t67: f64, t758: f64, t2408: f64, t2417: f64, t2426: f64, t2486: f64, t3683: f64, t3688: f64, t3690: f64, t3693: f64, t3695: f64) -> (f64, f64, f64, f64) {
    let t3813 = 0.24415263074675393405e-3_f64 * t1291 * t2663;
    let t3814 = t1284 * t67;
    let t3815 = t3814 * t758;
    let t3816 = 0.36622894612013090108e-3_f64 * t3815;
    let t3817 = t3813 - t2486 + t2408 + t2417 - t2426 - t3816 + t3688 + t3683 - t3690 - t3693 - t3695;
    (t3813, t3814, t3816, t3817)
}

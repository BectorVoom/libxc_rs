//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1014/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1014(t1579: f64, t2095: f64, t355: f64, t31477: f64, t171: f64, t5011: f64, t31479: f64, t2310: f64, t7780: f64, t31643: f64, t527: f64, t1418: f64, t7605: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t35646 = t2095 * t1579 * t355;
    let t35648 = 0.13073958333333333333e0_f64 * t31477;
    let t35649 = t171 * t5011;
    let t35653 = 0.13208198761633743869e-1_f64 * t31479;
    let t35662 = t7780 * t2310;
    let t35664 = t31643 * t527;
    let t35672 = t7605 * t1418;
    (t35646, t35648, t35649, t35653, t35662, t35664, t35672)
}

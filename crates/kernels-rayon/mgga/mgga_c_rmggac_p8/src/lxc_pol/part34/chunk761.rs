//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 761/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk761(t3352: f64, t68386: f64, t9205: f64, t14125: f64, t68455: f64, t8667: f64, t21709: f64, t8830: f64, t14117: f64, t8835: f64, t8842: f64, t15208: f64, t68922: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t73767 = t68386 * t3352 * t9205;
    let t73770 = t68455 * t14125 * t8667;
    let t73773 = t68455 * t21709 * t8830;
    let t73776 = t68455 * t14117 * t8835;
    let t73779 = t68455 * t14117 * t8842;
    let t73783 = t68922 * t15208;
    (t73767, t73770, t73773, t73776, t73779, t73783)
}

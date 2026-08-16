//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 722/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk722(t1128: f64, t510: f64, t1134: f64, t521: f64, t2821: f64, t2829: f64, t2834: f64, t2838: f64, t2875: f64, t2881: f64, t2922: f64, t2927: f64, t3661: f64, t3665: f64, t3669: f64, t3673: f64, t3677: f64, t3680: f64, t3684: f64, t3688: f64, t3698: f64, t3702: f64, t3706: f64, t3713: f64, t3714: f64) -> (f64, f64, f64) {
    let t3717 = t510 * t1128;
    let t3720 = t1134 * t521;
    let t3723 = -50.0_f64 / 9.0_f64 * t3661 * t3665 - 8.0_f64 / 9.0_f64 * t2821 * t3669 + 8.0_f64 / 9.0_f64 * t2829 * t3673 - 8.0_f64 / 3.0_f64 * t2834 * t3677 - 50.0_f64 / 3.0_f64 * t3680 * t3665 + 8.0_f64 / 3.0_f64 * t2838 * t3684 - 50.0_f64 / 3.0_f64 * t3688 * t3665 - 8.0_f64 / 3.0_f64 * t2834 * t3669 + 8.0_f64 / 3.0_f64 * t2838 * t3673 - 8.0_f64 / 9.0_f64 * t2821 * t3677 - 18.0_f64 * t2922 * t3698 + 21.0_f64 * t2875 * t3702 - 2.0_f64 * t2927 * t3706 - 2.0_f64 * t2927 * t3698 + 3.0_f64 * t2881 * t3702 - 100.0_f64 / 9.0_f64 * t3713 * t3714 - 100.0_f64 / 9.0_f64 * t3717 * t3714 - 100.0_f64 / 3.0_f64 * t3720 * t3714;
    (t3717, t3720, t3723)
}

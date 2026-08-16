//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 735/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk735(t1117: f64, t1123: f64, t1129: f64, t1134: f64, t1145: f64, t1536: f64, t1540: f64, t1543: f64, t1546: f64, t1549: f64, t2829: f64, t2868: f64, t2903: f64, t2922: f64, t3665: f64, t3684: f64, t3706: f64, t3714: f64, t3724: f64, t3727: f64, t3733: f64, t3739: f64, t3743: f64, t3747: f64, t3749: f64, t3753: f64, t3757: f64, t3760: f64, t3767: f64, t3772: f64, t3779: f64, t3786: f64, t3788: f64, t510: f64, t518: f64) -> f64 {
    let t3791 = -100.0_f64 / 3.0_f64 * t3724 * t3714 + 15.0_f64 * t2868 * t1145 * t3727 - 18.0_f64 * t2922 * t3706 - 50.0_f64 / 9.0_f64 * t3733 * t3665 + 8.0_f64 / 9.0_f64 * t2829 * t3684 - 32.0_f64 / 81.0_f64 * t3739 * t3743 - 16.0_f64 / 27.0_f64 * t3747 * t3749 - 32.0_f64 / 81.0_f64 * t3753 * t3743 - 16.0_f64 / 27.0_f64 * t3757 * t3749 - 36.0_f64 * t1134 * t3760 * t1129 - 36.0_f64 * t1134 * t1546 * t1123 + 42.0_f64 * t518 * t3767 * t1129 - 4.0_f64 * t1117 * t3772 - 4.0_f64 * t1117 * t1540 * t1123 + 6.0_f64 * t510 * t3779 + 30.0_f64 * t2903 * t1543 * t1123 + t3786 * t1549 + 2.0_f64 * t3788 * t1536;
    t3791
}

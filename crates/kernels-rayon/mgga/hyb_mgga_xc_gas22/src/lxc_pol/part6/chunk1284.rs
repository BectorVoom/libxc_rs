//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1284/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1284(t1804: f64, t1806: f64, t20129: f64, t20132: f64, t23674: f64, t23684: f64, t23688: f64, t23696: f64, t23701: f64, t23706: f64, t23726: f64, t23737: f64, t23740: f64, t23743: f64, t27728: f64, t27759: f64, t27766: f64, t27770: f64, t27777: f64, t27789: f64, t2970: f64, t457: f64, t559: f64, t7837: f64, t7866: f64, t7868: f64, t9839: f64) -> f64 {
    let t27797 = -t2970 * t27759 * t7837 / 6.0_f64 + t20129 / 216.0_f64 + t20132 / 288.0_f64 - t27766 / 72.0_f64 - t1804 * t27770 * t1806 * t559 * t457 / 6.0_f64 + t27777 / 216.0_f64 + t23674 / 54.0_f64 + t23684 / 24.0_f64 - t23688 / 36.0_f64 - t23696 / 72.0_f64 - 7.0_f64 / 216.0_f64 * t23701 - t23726 / 72.0_f64 - t23737 / 96.0_f64 - t23740 / 48.0_f64 - t23743 / 24.0_f64 + 7.0_f64 / 36.0_f64 * t27789 - 7.0_f64 / 72.0_f64 * t7866 * t23706 * t9839 - 7.0_f64 / 72.0_f64 * t7866 * t7868 * t27728;
    t27797
}

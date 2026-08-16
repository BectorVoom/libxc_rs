//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1723/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1723(t225: f64, t7824: f64, t1527: f64, t7106: f64, t2718: f64, t7823: f64, t798: f64, t25211: f64, t7815: f64, t1528: f64, t24297: f64, t25206: f64, t25209: f64, t25214: f64, t25218: f64, t25226: f64, t25230: f64, t259: f64, t2597: f64, t7842: f64, t855: f64, t866: f64) -> (f64, f64, f64, f64, f64) {
    let t26700 = t7824 * t225;
    let t26702 = t7106 * t1527;
    let t26703 = t2718 * t26702;
    let t26708 = t798 * t7823;
    let t26712 = 0.38381794893125283518e-1_f64 * t25211;
    let t26713 = t7815 * t225;
    let t26719 = -t26700 * t866 + 2.0_f64 * t855 * t26703 + 0.82246703342411321825e-2_f64 * t25206 - t2597 * t7842 + t26708 * t259 - t24297 * t1528 + 0.76763589786250567037e-1_f64 * t25209 + t26712 - t26713 * t866 - 0.16449340668482264365e-1_f64 * t25214 - 0.16449340668482264365e-1_f64 * t25218 - 0.16449340668482264365e-1_f64 * t25226 - 0.3289868133696452873e-1_f64 * t25230;
    (t26700, t26703, t26708, t26713, t26719)
}

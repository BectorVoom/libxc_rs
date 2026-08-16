//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1533/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1533(t11697: f64, t11710: f64, t3091: f64, t11725: f64, t828: f64, t11706: f64, t11660: f64, t2258: f64, t11779: f64, t3215: f64, t11231: f64, t11659: f64, t11672: f64, t11678: f64, t11696: f64, t11698: f64, t11707: f64, t11811: f64, t11859: f64, t16095: f64, t2857: f64, t3092: f64, t3117: f64, t3120: f64, t3211: f64, t43116: f64, t43121: f64, t4892: f64, t999: f64) -> f64 {
    let t43129 = t3091 * t11710 * t11697;
    let t43131 = t828 * t11725;
    let t43133 = t3091 * t43131 * t11706;
    let t43139 = t11660 * t2258;
    let t43146 = t11779 * t3215;
    let t43148 = 0.34299214494455789577e-2_f64 * t16095 * t3092 * t999 * t2857 * t11231 - 0.25724410870841842184e-2_f64 * t11859 * t3117 * t11659 * t43116 + 0.27439371595564631661e-1_f64 * t43121 * t3120 - 0.91464571985215438872e-2_f64 * t11672 * t11698 - 0.15244095330869239812e-1_f64 * t11672 * t11707 + 0.11433071498151929859e-2_f64 * t43129 + 0.19055119163586549765e-2_f64 * t43133 + 0.85748036236139473944e-3_f64 * t3091 * t3092 * t11678 * t11696 + 0.17149607247227894789e-2_f64 * t4892 * t3092 * t11659 * t43139 + 0.45732285992607719436e-2_f64 * t3211 * t11811 - 0.57927562257303111285e-1_f64 * t43146;
    t43148
}

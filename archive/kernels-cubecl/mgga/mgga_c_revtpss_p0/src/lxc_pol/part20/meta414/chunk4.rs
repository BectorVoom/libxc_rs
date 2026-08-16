//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1533/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1533<F: Float>(t11697: F, t11710: F, t3091: F, t11725: F, t828: F, t11706: F, t11660: F, t2258: F, t11779: F, t3215: F, t11231: F, t11659: F, t11672: F, t11678: F, t11696: F, t11698: F, t11707: F, t11811: F, t11859: F, t16095: F, t2857: F, t3092: F, t3117: F, t3120: F, t3211: F, t43116: F, t43121: F, t4892: F, t999: F) -> F {
    let t43129 = t3091 * t11710 * t11697;
    let t43131 = t828 * t11725;
    let t43133 = t3091 * t43131 * t11706;
    let t43139 = t11660 * t2258;
    let t43146 = t11779 * t3215;
    let t43148 = F::cast_from(0.34299214494455789577e-2_f64) * t16095 * t3092 * t999 * t2857 * t11231 - F::cast_from(0.25724410870841842184e-2_f64) * t11859 * t3117 * t11659 * t43116 + F::cast_from(0.27439371595564631661e-1_f64) * t43121 * t3120 - F::cast_from(0.91464571985215438872e-2_f64) * t11672 * t11698 - F::cast_from(0.15244095330869239812e-1_f64) * t11672 * t11707 + F::cast_from(0.11433071498151929859e-2_f64) * t43129 + F::cast_from(0.19055119163586549765e-2_f64) * t43133 + F::cast_from(0.85748036236139473944e-3_f64) * t3091 * t3092 * t11678 * t11696 + F::cast_from(0.17149607247227894789e-2_f64) * t4892 * t3092 * t11659 * t43139 + F::cast_from(0.45732285992607719436e-2_f64) * t3211 * t11811 - F::cast_from(0.57927562257303111285e-1_f64) * t43146;
    t43148
}

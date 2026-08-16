//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1289/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1289(t18711: f64, t18740: f64, t18747: f64, t18851: f64, t2257: f64, t2279: f64, t2281: f64, t2297: f64, t2318: f64, t3088: f64, t3107: f64, t3121: f64, t3136: f64, t3139: f64, t3140: f64, t6122: f64, t6266: f64, t6272: f64, t6282: f64, t6300: f64, t6308: f64, t6341: f64, t6345: f64, t8067: f64, t8068: f64, t8120: f64, t8129: f64, t8139: f64, t8150: f64, t8161: f64, t8171: f64, t8211: f64, t870: f64) -> f64 {
    let t22623 = -0.35089341735807877242e1_f64 * t18711 * t3121 + 0.51947577317044391277e2_f64 * t18740 * t3140 - 0.70178683471615754484e1_f64 * t6266 * t8161 + 0.10389515463408878255e3_f64 * t6300 * t8171 - 6.0_f64 * t8211 * t6341 + 0.96491876992155210402e2_f64 * t8120 * t6345 - 6.0_f64 * t18851 * t3088 + 0.96491876992155210402e2_f64 * t18747 * t3107 - 12.0_f64 * t6272 * t8129 + 0.19298375398431042081e3_f64 * t6308 * t8139 - 6.0_f64 * t2257 * t8068 * t870 + 0.96491876992155210402e2_f64 * t2279 * t8067 * t2281 * t870 + 0.10526802520742363173e2_f64 * t6300 * t8150 + 0.10526802520742363173e2_f64 * t2318 * t3136 * t2297 + 0.6233709278045326953e3_f64 * t6282 * t3139 * t6122;
    t22623
}

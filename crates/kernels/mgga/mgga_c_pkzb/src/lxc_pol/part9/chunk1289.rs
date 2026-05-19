//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1289/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1289<F: Float>(t18711: F, t18740: F, t18747: F, t18851: F, t2257: F, t2279: F, t2281: F, t2297: F, t2318: F, t3088: F, t3107: F, t3121: F, t3136: F, t3139: F, t3140: F, t6122: F, t6266: F, t6272: F, t6282: F, t6300: F, t6308: F, t6341: F, t6345: F, t8067: F, t8068: F, t8120: F, t8129: F, t8139: F, t8150: F, t8161: F, t8171: F, t8211: F, t870: F) -> F {
    let t22623 = -F::cast_from(0.35089341735807877242e1_f64) * t18711 * t3121 + F::cast_from(0.51947577317044391277e2_f64) * t18740 * t3140 - F::cast_from(0.70178683471615754484e1_f64) * t6266 * t8161 + F::cast_from(0.10389515463408878255e3_f64) * t6300 * t8171 - F::new(6.0) * t8211 * t6341 + F::cast_from(0.96491876992155210402e2_f64) * t8120 * t6345 - F::new(6.0) * t18851 * t3088 + F::cast_from(0.96491876992155210402e2_f64) * t18747 * t3107 - F::new(12.0) * t6272 * t8129 + F::cast_from(0.19298375398431042081e3_f64) * t6308 * t8139 - F::new(6.0) * t2257 * t8068 * t870 + F::cast_from(0.96491876992155210402e2_f64) * t2279 * t8067 * t2281 * t870 + F::cast_from(0.10526802520742363173e2_f64) * t6300 * t8150 + F::cast_from(0.10526802520742363173e2_f64) * t2318 * t3136 * t2297 + F::cast_from(0.6233709278045326953e3_f64) * t6282 * t3139 * t6122;
    t22623
}

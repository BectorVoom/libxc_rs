//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1386/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1386<F: Float>(t2281: F, t9958: F, t3102: F, t862: F, t9888: F, t2278: F, t3774: F, t10006: F, t10009: F, t10013: F, t10016: F, t10020: F, t1197: F, t18711: F, t18854: F, t18875: F, t18882: F, t2257: F, t2274: F, t22757: F, t2279: F, t2282: F, t2291: F, t27516: F, t3103: F, t3116: F, t3807: F, t3820: F, t6272: F, t6294: F, t6308: F, t8099: F, t8115: F, t870: F, t872: F, t9891: F, t9930: F, t9959: F, t9993: F) -> (F, F) {
    let t27812 = t9958 * t2281;
    let t27830 = t3102 * t3102;
    let t27834 = t9888 * t862;
    let t27839 = t3774 * t2278;
    let t27846 = 0.20508037716432813316e4 * t18875 * t9993 - 4.0 * t6272 * t10009 + 0.64327917994770140268e2 * t6308 * t10013 - 4.0 * t2257 * t9959 * t870 + 0.64327917994770140268e2 * t2279 * t27812 * t870 - 0.38596750796862084162e3 * t18854 * t10006 + 0.12865583598954028054e3 * t6308 * t10016 + 0.4138081033541872024e4 * t18882 * t10020 + 0.11696447245269292414e1 * t3116 * t8099 - 0.11696447245269292414e1 * t18711 * t3807 + 0.5848223622634646207e0 * t6294 * t3820 + 0.11696447245269292414e1 * t2291 * t9930 + 0.64327917994770140268e2 * t2279 * t27830 * t2281 + 2.0 * t27834 * t872 + 1.0 * t9891 * t2274 + 0.32163958997385070134e2 * t27839 * t2282 + 2.0 * t22757 * t1197 + 4.0 * t8115 * t3103 + t27516;
    (t27830, t27846)
}

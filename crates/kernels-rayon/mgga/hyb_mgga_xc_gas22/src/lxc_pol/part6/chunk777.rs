//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 777/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk777(t2289: f64, t4180: f64, t848: f64, t4193: f64, t839: f64, t2311: f64, t2314: f64, t1379: f64, t260: f64, t3430: f64, t4110: f64, t4112: f64, t4116: f64, t4142: f64, t4145: f64, t4176: f64, t4200: f64, t856: f64) -> (f64, f64, f64, f64, f64) {
    let t4207 = t2289 * t4180 * t848;
    let t4211 = t839 * t4193 * t848;
    let t4214 = t2311 * t4180;
    let t4215 = t4214 * t2314;
    let t4218 = -t4110 + t4112 - t4116 + t4142 + t4145 + t260 * t4200 + 0.19751673498613801407e-1_f64 * t260 * t4176 - 0.11696447245269292414e1_f64 * t3430 * t1379 + 0.11696447245269292414e1_f64 * t856 * t4207 - 0.5848223622634646207e0_f64 * t856 * t4211 - 0.17315859105681463759e2_f64 * t856 * t4215;
    (t4207, t4211, t4214, t4215, t4218)
}

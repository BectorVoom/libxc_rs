//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 939/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk939(t10213: f64, t10214: f64, t3730: f64, t919: f64, t921: f64, t2381: f64, t3757: f64, t6366: f64, t179: f64, t932: f64, t9795: f64, t10148: f64, t10192: f64, t10197: f64, t10201: f64, t10205: f64, t10209: f64, t1238: f64, t2380: f64, t3185: f64, t3206: f64, t3242: f64, t3860: f64, t404: f64, t6430: f64, t6449: f64, t8331: f64, t8340: f64, t8342: f64, t8364: f64, t918: f64, t923: f64, t934: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10215 = t10213 * t10214;
    let t10220 = t3730 * t919 * t921;
    let t10221 = t2381 * t10220;
    let t10225 = t3757 * t919 * t921;
    let t10226 = t6366 * t10225;
    let t10236 = t179 * t932 * t9795;
    let t10239 = 11.0_f64 / 324.0_f64 * t10148 + 0.21437009059034868486e-3_f64 * t918 * t10192 + 0.72409452821628889107e-2_f64 * t10197 * t923 + 0.14291339372689912324e-3_f64 * t10201 - t8331 + t8340 / 216.0_f64 + 0.85748036236139473944e-3_f64 * t3206 * t10205 - 0.17149607247227894789e-2_f64 * t3185 * t10209 + 0.25724410870841842183e-2_f64 * t2380 * t10215 + t8342 / 81.0_f64 - 0.42874018118069736972e-3_f64 * t2380 * t10221 + 0.12862205435420921092e-2_f64 * t2380 * t10226 + t6430 - 0.95275595817932748827e-4_f64 * t8364 + t6449 / 432.0_f64 - 0.14481890564325777821e-1_f64 * t3860 * t934 + 0.45732285992607719436e-2_f64 * t1238 * t3242 - 0.42874018118069736972e-3_f64 * t404 * t10236;
    (t10215, t10220, t10221, t10225, t10226, t10236, t10239)
}

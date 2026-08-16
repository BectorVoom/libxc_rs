//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 939/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk939<F: Float>(t10213: F, t10214: F, t3730: F, t919: F, t921: F, t2381: F, t3757: F, t6366: F, t179: F, t932: F, t9795: F, t10148: F, t10192: F, t10197: F, t10201: F, t10205: F, t10209: F, t1238: F, t2380: F, t3185: F, t3206: F, t3242: F, t3860: F, t404: F, t6430: F, t6449: F, t8331: F, t8340: F, t8342: F, t8364: F, t918: F, t923: F, t934: F) -> (F, F, F, F, F, F, F) {
    let t10215 = t10213 * t10214;
    let t10220 = t3730 * t919 * t921;
    let t10221 = t2381 * t10220;
    let t10225 = t3757 * t919 * t921;
    let t10226 = t6366 * t10225;
    let t10236 = t179 * t932 * t9795;
    let t10239 = F::cast_from(11.0_f64) / F::cast_from(324.0_f64) * t10148 + F::cast_from(0.21437009059034868486e-3_f64) * t918 * t10192 + F::cast_from(0.72409452821628889107e-2_f64) * t10197 * t923 + F::cast_from(0.14291339372689912324e-3_f64) * t10201 - t8331 + t8340 / F::cast_from(216.0_f64) + F::cast_from(0.85748036236139473944e-3_f64) * t3206 * t10205 - F::cast_from(0.17149607247227894789e-2_f64) * t3185 * t10209 + F::cast_from(0.25724410870841842183e-2_f64) * t2380 * t10215 + t8342 / F::cast_from(81.0_f64) - F::cast_from(0.42874018118069736972e-3_f64) * t2380 * t10221 + F::cast_from(0.12862205435420921092e-2_f64) * t2380 * t10226 + t6430 - F::cast_from(0.95275595817932748827e-4_f64) * t8364 + t6449 / F::cast_from(432.0_f64) - F::cast_from(0.14481890564325777821e-1_f64) * t3860 * t934 + F::cast_from(0.45732285992607719436e-2_f64) * t1238 * t3242 - F::cast_from(0.42874018118069736972e-3_f64) * t404 * t10236;
    (t10215, t10220, t10221, t10225, t10226, t10236, t10239)
}

//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1313/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1313(t3199: f64, t6517: f64, t11445: f64, t179: f64, t19203: f64, t404: f64, t11345: f64, t17938: f64, t11390: f64, t2380: f64, t6475: f64, t10075: f64, t10077: f64, t10092: f64, t10123: f64, t11500: f64, t18969: f64, t22972: f64, t2370: f64, t2381: f64, t2382: f64, t26975: f64, t27083: f64, t28064: f64, t28188: f64, t3026: f64, t3185: f64, t3187: f64, t3225: f64, t3913: f64, t406: f64, t6411: f64, t8428: f64, t8435: f64, t8450: f64) -> (f64, f64, f64) {
    let t31827 = t6517 * t3199;
    let t31838 = t404 * t179 * t19203 * t11445;
    let t31857 = t11345 * t17938;
    let t31865 = t2380 * t6475 * t11390;
    let t31868 = -0.25724410870841842184e-2_f64 * t8428 * t2381 * t11500 * t18969 + 0.38586616306262763276e-2_f64 * t8428 * t406 * t10075 * t31827 - 0.42874018118069736972e-3_f64 * t8450 * t2381 * t11500 * t2382 - 0.34299214494455789579e-2_f64 * t31838 - 0.20579528696673473746e-1_f64 * t28064 * t10123 + 0.20579528696673473746e-1_f64 * t28188 * t10077 - 0.25724410870841842183e-2_f64 * t3185 * t2381 * t3913 * t2370 * t3026 + 0.25724410870841842184e-2_f64 * t8435 * t2381 * t11500 * t6411 - 0.38586616306262763276e-2_f64 * t8435 * t406 * t10075 * t10092 + 0.30011812682648815881e-2_f64 * t22972 * t406 * t31857 * t3187 - 0.43445671692977333464e-1_f64 * t26975 * t3225 - 0.85748036236139473947e-3_f64 * t31865 + 0.25724410870841842183e-2_f64 * t27083;
    (t31827, t31857, t31868)
}

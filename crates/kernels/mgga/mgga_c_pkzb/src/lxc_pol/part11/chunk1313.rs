//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1313/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1313<F: Float>(t3199: F, t6517: F, t11445: F, t179: F, t19203: F, t404: F, t11345: F, t17938: F, t11390: F, t2380: F, t6475: F, t10075: F, t10077: F, t10092: F, t10123: F, t11500: F, t18969: F, t22972: F, t2370: F, t2381: F, t2382: F, t26975: F, t27083: F, t28064: F, t28188: F, t3026: F, t3185: F, t3187: F, t3225: F, t3913: F, t406: F, t6411: F, t8428: F, t8435: F, t8450: F) -> (F, F, F) {
    let t31827 = t6517 * t3199;
    let t31838 = t404 * t179 * t19203 * t11445;
    let t31857 = t11345 * t17938;
    let t31865 = t2380 * t6475 * t11390;
    let t31868 = -F::new(0.25724410870841842184e-2) * t8428 * t2381 * t11500 * t18969 + F::new(0.38586616306262763276e-2) * t8428 * t406 * t10075 * t31827 - F::new(0.42874018118069736972e-3) * t8450 * t2381 * t11500 * t2382 - F::new(0.34299214494455789579e-2) * t31838 - F::new(0.20579528696673473746e-1) * t28064 * t10123 + F::new(0.20579528696673473746e-1) * t28188 * t10077 - F::new(0.25724410870841842183e-2) * t3185 * t2381 * t3913 * t2370 * t3026 + F::new(0.25724410870841842184e-2) * t8435 * t2381 * t11500 * t6411 - F::new(0.38586616306262763276e-2) * t8435 * t406 * t10075 * t10092 + F::new(0.30011812682648815881e-2) * t22972 * t406 * t31857 * t3187 - F::new(0.43445671692977333464e-1) * t26975 * t3225 - F::new(0.85748036236139473947e-3) * t31865 + F::new(0.25724410870841842183e-2) * t27083;
    (t31827, t31857, t31868)
}

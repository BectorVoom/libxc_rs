//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 352/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk352<F: Float>(t2890: F, t590: F, t2783: F, t549: F, t1: F, t2760: F, t544: F, t1424: F, t1429: F, t1441: F, t1450: F, t1537: F, t1562: F, t1572: F, t1646: F, t2386: F, t2460: F, t2468: F, t2472: F, t2474: F, t2480: F, t2484: F, t2490: F, t2851: F, t2856: F, t2859: F, t2862: F, t2865: F, t2869: F, t2872: F, t2877: F, t2887: F, t536: F, t567: F, t597: F) -> (F, F, F) {
    let t2891 = t2890 * t590;
    let t2894 = t549 * t2783;
    let t2897 = t2760 * t1;
    let t2898 = t544 * t2897;
    let t2901 = F::cast_from(0.11502877786176224903e2_f64) * t597 * t2851 - F::cast_from(0.69017266717057349418e1_f64) * t1562 * t2856 - F::cast_from(0.10725146985555128001e1_f64) * t2859 * t2386 + F::cast_from(0.71500979903700853338e0_f64) * t1572 * t2862 + F::cast_from(0.23005755572352449806e1_f64) * t567 * t2865 - F::cast_from(0.23005755572352449806e1_f64) * t1450 * t2869 - F::cast_from(0.35750489951850426669e0_f64) * t2872 * t1646 + F::cast_from(0.35750489951850426669e0_f64) * t536 * t2877 - F::cast_from(0.19171462976960374838e0_f64) * t2460 + F::cast_from(0.42603251059911944084e-1_f64) * t2468 - F::cast_from(0.29792074959875355558e-1_f64) * t2472 + F::cast_from(0.29792074959875355558e-1_f64) * t2474 + F::cast_from(0.19171462976960374838e0_f64) * t2480 - F::cast_from(0.38342925953920749676e0_f64) * t2484 + F::cast_from(0.38342925953920749676e0_f64) * t2490 + F::cast_from(0.51123901271894332902e0_f64) * t1441 * t2887 - F::cast_from(0.51123901271894332902e0_f64) * t1537 * t2891 + F::cast_from(0.39722766613167140743e-1_f64) * t1429 * t2894 - F::cast_from(0.39722766613167140743e-1_f64) * t2898 * t1424;
    (t2897, t2898, t2901)
}

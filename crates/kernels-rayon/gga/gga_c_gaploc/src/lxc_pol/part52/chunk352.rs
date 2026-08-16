//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 352/1013 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk352(t2890: f64, t590: f64, t2783: f64, t549: f64, t1: f64, t2760: f64, t544: f64, t1424: f64, t1429: f64, t1441: f64, t1450: f64, t1537: f64, t1562: f64, t1572: f64, t1646: f64, t2386: f64, t2460: f64, t2468: f64, t2472: f64, t2474: f64, t2480: f64, t2484: f64, t2490: f64, t2851: f64, t2856: f64, t2859: f64, t2862: f64, t2865: f64, t2869: f64, t2872: f64, t2877: f64, t2887: f64, t536: f64, t567: f64, t597: f64) -> (f64, f64, f64, f64) {
    let t2891 = t2890 * t590;
    let t2894 = t549 * t2783;
    let t2897 = t2760 * t1;
    let t2898 = t544 * t2897;
    let t2901 = 0.11502877786176224903e2_f64 * t597 * t2851 - 0.69017266717057349418e1_f64 * t1562 * t2856 - 0.10725146985555128001e1_f64 * t2859 * t2386 + 0.71500979903700853338e0_f64 * t1572 * t2862 + 0.23005755572352449806e1_f64 * t567 * t2865 - 0.23005755572352449806e1_f64 * t1450 * t2869 - 0.35750489951850426669e0_f64 * t2872 * t1646 + 0.35750489951850426669e0_f64 * t536 * t2877 - 0.19171462976960374838e0_f64 * t2460 + 0.42603251059911944084e-1_f64 * t2468 - 0.29792074959875355558e-1_f64 * t2472 + 0.29792074959875355558e-1_f64 * t2474 + 0.19171462976960374838e0_f64 * t2480 - 0.38342925953920749676e0_f64 * t2484 + 0.38342925953920749676e0_f64 * t2490 + 0.51123901271894332902e0_f64 * t1441 * t2887 - 0.51123901271894332902e0_f64 * t1537 * t2891 + 0.39722766613167140743e-1_f64 * t1429 * t2894 - 0.39722766613167140743e-1_f64 * t2898 * t1424;
    (t2894, t2897, t2898, t2901)
}

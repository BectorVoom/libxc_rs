//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 529/1268 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk529<F: Float>(t1424: F, t1429: F, t1441: F, t1450: F, t1537: F, t1562: F, t1572: F, t1646: F, t2386: F, t2460: F, t2468: F, t2472: F, t2474: F, t2480: F, t2484: F, t2490: F, t2851: F, t2856: F, t2859: F, t2862: F, t2865: F, t2869: F, t2872: F, t2877: F, t2887: F, t2891: F, t2894: F, t2898: F, t536: F, t567: F, t597: F) -> (F,) {
    let t2901 = 0.11502877786176224903e2 * t597 * t2851 - 0.69017266717057349418e1 * t1562 * t2856 - 0.10725146985555128001e1 * t2859 * t2386 + 0.71500979903700853338e0 * t1572 * t2862 + 0.23005755572352449806e1 * t567 * t2865 - 0.23005755572352449806e1 * t1450 * t2869 - 0.35750489951850426669e0 * t2872 * t1646 + 0.35750489951850426669e0 * t536 * t2877 - 0.19171462976960374838e0 * t2460 + 0.42603251059911944084e-1 * t2468 - 0.29792074959875355558e-1 * t2472 + 0.29792074959875355558e-1 * t2474 + 0.19171462976960374838e0 * t2480 - 0.38342925953920749676e0 * t2484 + 0.38342925953920749676e0 * t2490 + 0.51123901271894332902e0 * t1441 * t2887 - 0.51123901271894332902e0 * t1537 * t2891 + 0.39722766613167140743e-1 * t1429 * t2894 - 0.39722766613167140743e-1 * t2898 * t1424;
    (t2901,)
}

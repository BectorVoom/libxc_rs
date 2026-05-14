//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 728/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk728<F: Float>(t11956: F, t11981: F, t12156: F, t12338: F, t2041: F, t5525: F, t2038: F, t5531: F, t2040: F, t798: F, t2049: F, t5533: F, t5552: F, t11709: F, t11712: F, t11715: F, t11718: F, t11721: F, t11724: F, t11728: F, t11732: F, t11736: F, t11739: F, t11742: F, t11745: F, t11747: F, t11751: F, t11754: F, t11756: F, t11758: F, t11760: F) -> (F, F, F, F, F, F, F) {
    let t12340 = t11956 + t11981 + t12156 + t12338;
    let t12342 = t5525 * t2041;
    let t12345 = t2038 * t5531;
    let t12350 = t2040 * t2040;
    let t12351 = 1.0 / t12350;
    let t12352 = t798 * t12351;
    let t12353 = t5533 * t2049;
    let t12356 = t2049 * t5552;
    let t12377 = -0.1875e0 * t11709 + 0.375e0 * t11712 - 0.1875e0 * t11715 + 0.80937499999999999999e-1 * t11718 + 0.5625e0 * t11721 - 0.80937499999999999999e-1 * t11724 - 0.101171875e-1 * t11728 + 0.5625e0 * t11732 + 0.101171875e-1 * t11736 + 0.625e-1 * t11739 - 0.60703125e-1 * t11742 + 0.303515625e-1 * t11745 - 0.13489583333333333333e-1 * t11747 + 0.625e-1 * t11751 + 0.40468749999999999999e-1 * t11754 - 0.40468749999999999999e-1 * t11756 + 0.1875e0 * t11758 - 0.40468749999999999999e-1 * t11760;
    (t12340, t12342, t12345, t12352, t12353, t12356, t12377)
}

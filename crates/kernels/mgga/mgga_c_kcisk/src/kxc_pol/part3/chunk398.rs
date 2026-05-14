//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 398/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk398<F: Float>(t45: F, t857: F, t890: F, t98: F, t896: F, t898: F, t2958: F, t2960: F, t2962: F, t2967: F, t2969: F, t2971: F, t891: F, t101: F, t102: F, t119: F, t157: F, t172: F, t2942: F, t2943: F, t2949: F, t2951: F, t2974: F, t2979: F, t2982: F, t67: F, t69: F, t863: F, t881: F, t884: F, t889: F, t89: F, t899: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t2989 = t45 * t857;
    let t2993 = t890 * t98;
    let t2994 = 1.0 / t2993;
    let t2995 = t896 * t896;
    let t2997 = t2994 * t2995 * t898;
    let t3006 = -0.57538888888888888889e0 * t2958 + 0.11507777777777777778e1 * t2960 + 0.40256666666666666667e0 * t2962 + 0.366775e-1 * t2967 + 0.73355e-1 * t2969 + 0.137975e0 * t2971;
    let t3008 = t891 * t3006 * t898;
    let t3011 = t890 * t890;
    let t3012 = 1.0 / t3011;
    let t3013 = t3012 * t2995;
    let t3014 = t101 * t101;
    let t3015 = 1.0 / t3014;
    let t3016 = t3013 * t3015;
    let t3020 = t67 * (-0.14764770444444444444e-2 * t857 * t172 * t89 - 0.35616666666666666667e-1 * t2942 * t2943 * t881 - 2.0 * t2949 * t2951 + 1.0 * t863 * t2974 + 0.16081824322151104822e2 * t2979 * t2982 + 0.24415406715670879921e-3 * t884 * t69 * t119 * t102 + 0.10843580882781524214e-1 * t2989 * t157 * t899 + 0.11696446794910408142e1 * t889 * t2997 - 0.58482233974552040708e0 * t889 * t3008 - 0.17315755899375863299e2 * t889 * t3016);
    (t2989, t2994, t2995, t2997, t3006, t3008, t3011, t3012, t3014, t3015, t3016, t3020)
}

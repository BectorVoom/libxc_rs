//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 419/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk419(t3006: f64, t891: f64, t898: f64, t890: f64, t2995: f64, t101: f64, t102: f64, t119: f64, t157: f64, t172: f64, t2942: f64, t2943: f64, t2949: f64, t2951: f64, t2974: f64, t2979: f64, t2982: f64, t2989: f64, t2997: f64, t67: f64, t69: f64, t857: f64, t863: f64, t881: f64, t884: f64, t889: f64, t89: f64, t899: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3008 = t891 * t3006 * t898;
    let t3011 = t890 * t890;
    let t3012 = 1.0_f64 / t3011;
    let t3013 = t3012 * t2995;
    let t3014 = t101 * t101;
    let t3015 = 1.0_f64 / t3014;
    let t3016 = t3013 * t3015;
    let t3020 = t67 * (-0.14764770444444444444e-2_f64 * t857 * t172 * t89 - 0.35616666666666666667e-1_f64 * t2942 * t2943 * t881 - 2.0_f64 * t2949 * t2951 + 1.0_f64 * t863 * t2974 + 0.16081824322151104822e2_f64 * t2979 * t2982 + 0.24415406715670879921e-3_f64 * t884 * t69 * t119 * t102 + 0.10843580882781524214e-1_f64 * t2989 * t157 * t899 + 0.11696446794910408142e1_f64 * t889 * t2997 - 0.58482233974552040708e0_f64 * t889 * t3008 - 0.17315755899375863299e2_f64 * t889 * t3016);
    (t3008, t3011, t3012, t3014, t3015, t3016, t3020)
}

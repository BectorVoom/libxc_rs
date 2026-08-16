//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2206/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2206(t25604: f64, t7825: f64, t1678: f64, t7150: f64, t8521: f64, t27418: f64, t3057: f64, t3046: f64, t7810: f64, t27543: f64, t994: f64, t1000: f64, t1043: f64, t1089: f64, t1096: f64, t1668: f64, t25464: f64, t25593: f64, t25607: f64, t25611: f64, t25613: f64, t25683: f64, t27411: f64, t27433: f64, t27437: f64, t27621: f64, t27683: f64, t27687: f64, t3059: f64, t7144: f64, t7145: f64, t7159: f64, t7160: f64, t7167: f64, t7817: f64, t7833: f64, t93497: f64, t93498: f64, t93521: f64, t93939: f64, t93963: f64, t94042: f64, t94053: f64, t988: f64) -> f64 {
    let t99909 = t7825 * t25604;
    let t99914 = t7150 * t1678;
    let t99915 = t99914 * t8521;
    let t99934 = t3057 * t27418;
    let t99940 = t3046 * t7810;
    let t99947 = t994 * t27543;
    let t99950 = -0.52041769129231196772e1_f64 * t94053 * t7145 * t7817 * t3059 - 0.8673628188205199462e0_f64 * t93521 * t7833 + 0.17347256376410398924e1_f64 * t99909 * t25607 + 0.17347256376410398924e1_f64 * t93963 * t27437 + 0.17347256376410398924e1_f64 * t99915 * t25613 - 0.8673628188205199462e0_f64 * t27621 * t25683 - 0.4336814094102599731e0_f64 * t7167 * t93939 * t1668 * t1089 + 0.34694512752820797848e1_f64 * t7144 * t7160 * t27411 * t988 - 0.17347256376410398924e1_f64 * t94042 * t27433 - 0.52041769129231196772e1_f64 * t7159 * t25464 * t27411 * t1096 + 0.34694512752820797848e1_f64 * t99934 * t25593 - 0.34694512752820797848e1_f64 * t93497 * t27683 * t93498 - 0.13170898365871023197e1_f64 * t99940 * t1000 + 0.17347256376410398924e1_f64 * t25611 * t27687 * t1043 * t1089 - 0.13170898365871023197e1_f64 * t99947 * t1000;
    t99950
}

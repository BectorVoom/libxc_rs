//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2206/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2206<F: Float>(t25604: F, t7825: F, t1678: F, t7150: F, t8521: F, t27418: F, t3057: F, t3046: F, t7810: F, t27543: F, t994: F, t1000: F, t1043: F, t1089: F, t1096: F, t1668: F, t25464: F, t25593: F, t25607: F, t25611: F, t25613: F, t25683: F, t27411: F, t27433: F, t27437: F, t27621: F, t27683: F, t27687: F, t3059: F, t7144: F, t7145: F, t7159: F, t7160: F, t7167: F, t7817: F, t7833: F, t93497: F, t93498: F, t93521: F, t93939: F, t93963: F, t94042: F, t94053: F, t988: F) -> F {
    let t99909 = t7825 * t25604;
    let t99914 = t7150 * t1678;
    let t99915 = t99914 * t8521;
    let t99934 = t3057 * t27418;
    let t99940 = t3046 * t7810;
    let t99947 = t994 * t27543;
    let t99950 = -F::cast_from(0.52041769129231196772e1_f64) * t94053 * t7145 * t7817 * t3059 - F::cast_from(0.8673628188205199462e0_f64) * t93521 * t7833 + F::cast_from(0.17347256376410398924e1_f64) * t99909 * t25607 + F::cast_from(0.17347256376410398924e1_f64) * t93963 * t27437 + F::cast_from(0.17347256376410398924e1_f64) * t99915 * t25613 - F::cast_from(0.8673628188205199462e0_f64) * t27621 * t25683 - F::cast_from(0.4336814094102599731e0_f64) * t7167 * t93939 * t1668 * t1089 + F::cast_from(0.34694512752820797848e1_f64) * t7144 * t7160 * t27411 * t988 - F::cast_from(0.17347256376410398924e1_f64) * t94042 * t27433 - F::cast_from(0.52041769129231196772e1_f64) * t7159 * t25464 * t27411 * t1096 + F::cast_from(0.34694512752820797848e1_f64) * t99934 * t25593 - F::cast_from(0.34694512752820797848e1_f64) * t93497 * t27683 * t93498 - F::cast_from(0.13170898365871023197e1_f64) * t99940 * t1000 + F::cast_from(0.17347256376410398924e1_f64) * t25611 * t27687 * t1043 * t1089 - F::cast_from(0.13170898365871023197e1_f64) * t99947 * t1000;
    t99950
}

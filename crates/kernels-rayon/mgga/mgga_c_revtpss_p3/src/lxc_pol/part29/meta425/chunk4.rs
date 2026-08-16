//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1568/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1568(t1757: f64, t3515: f64, t3497: f64, t5184: f64, t3523: f64, t5180: f64, t1187: f64, t12429: f64, t12470: f64, t12481: f64, t12486: f64, t12491: f64, t16955: f64, t16959: f64, t16962: f64, t16966: f64, t16971: f64, t16974: f64, t16979: f64, t3477: f64, t3496: f64, t3521: f64, t5163: f64, t5185: f64) -> f64 {
    let t16982 = t1757 * t3515;
    let t16985 = t5184 * t3497;
    let t16988 = t5180 * t3523;
    let t16989 = t16988 * t1187;
    let t16992 = t5184 * t3515;
    let t16995 = -0.19298375398431042081e3_f64 * t12429 * t16955 + 0.64327917994770140268e2_f64 * t3477 * t16959 + 0.32163958997385070134e2_f64 * t3477 * t16962 + 0.2069040516770936012e4_f64 * t12470 * t16966 - 0.23392894490538584828e1_f64 * t12491 * t5163 + 0.35089341735807877242e1_f64 * t3521 * t16971 + 6.0_f64 * t3477 * t16974 + 0.34631718211362927518e2_f64 * t12481 * t5185 - 0.23392894490538584828e1_f64 * t3496 * t16979 - 0.11696447245269292414e1_f64 * t3496 * t16982 - 0.10389515463408878255e3_f64 * t12486 * t16985 + 0.34631718211362927518e2_f64 * t3521 * t16989 + 0.17315859105681463759e2_f64 * t3521 * t16992;
    t16995
}

//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1212/1274 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk1212<F: Float>(t1187: F, t16988: F, t3515: F, t5184: F, t12429: F, t12470: F, t12481: F, t12486: F, t12491: F, t16955: F, t16959: F, t16962: F, t16966: F, t16971: F, t16974: F, t16979: F, t16982: F, t16985: F, t3477: F, t3496: F, t3521: F, t5163: F, t5185: F) -> (F,) {
    let t16989 = t16988 * t1187;
    let t16992 = t5184 * t3515;
    let t16995 = -0.19298375398431042081e3 * t12429 * t16955 + 0.64327917994770140268e2 * t3477 * t16959 + 0.32163958997385070134e2 * t3477 * t16962 + 0.2069040516770936012e4 * t12470 * t16966 - 0.23392894490538584828e1 * t12491 * t5163 + 0.35089341735807877242e1 * t3521 * t16971 + 6.0 * t3477 * t16974 + 0.34631718211362927518e2 * t12481 * t5185 - 0.23392894490538584828e1 * t3496 * t16979 - 0.11696447245269292414e1 * t3496 * t16982 - 0.10389515463408878255e3 * t12486 * t16985 + 0.34631718211362927518e2 * t3521 * t16989 + 0.17315859105681463759e2 * t3521 * t16992;
    (t16995,)
}

//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 945/1274 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk945<F: Float>(t2648: F, t2741: F, t2710: F, t826: F, t9732: F, t234: F, t2735: F, t10631: F, t808: F, t2699: F, t798: F, t802: F, t2703: F, t2707: F, t159: F, t853: F) -> (F, F, F, F, F, F, F, F) {
    let t10881 = t2741 * t2648;
    let t10885 = 0.81322168495418382223e-4 * t2710 * t9732 * t826;
    let t10886 = t2735 * t234;
    let t10887 = t808 * t10631;
    let t10888 = t10886 * t10887;
    let t10890 = t2699 * t798;
    let t10891 = t10890 * t802;
    let t10893 = t2703 * t2707;
    let t10899 = t159 * t853;
    (t10881, t10885, t10886, t10888, t10890, t10891, t10893, t10899)
}

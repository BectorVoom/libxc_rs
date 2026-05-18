//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1212/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk1212<F: Float>(t11387: F, t6109: F, t934: F, t11385: F, t6158: F, t953: F, t1622: F, t4669: F, t6177: F, t6174: F, t2970: F, t6173: F) -> (F, F, F, F, F, F) {
    let t19255 = t6109 * t11387;
    let t19256 = t19255 * t934;
    let t19258 = F::new(0.51726012919273400301e3) * t11385 * t19256;
    let t19263 = t6158 * t953;
    let t19266 = t1622 * t4669;
    let t19269 = t6177 * t953;
    let t19272 = t6174 * t953;
    let t19275 = t6173 * t2970;
    (t19258, t19263, t19266, t19269, t19272, t19275)
}

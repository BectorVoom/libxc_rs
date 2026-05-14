//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1189/1212 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1189<F: Float>(t213: F, t225: F, t25921: F, t25930: F, t25931: F, t25933: F, t25934: F, t25961: F, t27868: F, t27980: F, t46422: F, t561: F, t94574: F, t94694: F, t94700: F, t94703: F, t94705: F, t94714: F, t94716: F, t94721: F, t94726: F, t94729: F, t94733: F, t94735: F, t94737: F) -> (F,) {
    let t94744 = 0.32927245914677557992e-1 * t94694 + t94700 - t94703 - 0.52041769129231196772e1 * t94705 * t25934 + 0.65854491829355115987e0 * t213 * t94574 * t225 * t561 + 0.26020884564615598386e1 * t25921 * t25961 - 0.21951497276451705329e-1 * t94714 - 0.52041769129231196772e1 * t25930 * t94716 * t25933 - 0.26020884564615598386e1 * t25930 * t25931 * t94721 - 0.34697458558045176417e-2 * t94726 - 0.32927245914677557992e-1 * t94729 - 0.19514881078765566038e-2 * t94733 - 0.39029762157531132076e-1 * t94735 + 0.52041769129231196772e1 * t25930 * t27980 * t94737 - 0.26020884564615598386e1 * t27868 * t27980 * t46422;
    (t94744,)
}

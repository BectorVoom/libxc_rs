//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 1105/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk1105<F: Float>(t40702: F, t8571: F, t40081: F, t46434: F, t7198: F, t46438: F, t7204: F, t37018: F, t42234: F, t42239: F, t42243: F, t42247: F, t42248: F, t42250: F, t42259: F, t46563: F, t48027: F, t48029: F, t48031: F, t48036: F, t48039: F, t739: F) -> F {
    let t48041 = t8571 * t40702;
    let t48043 = t8571 * t40081;
    let t48047 = t7198 * t46434;
    let t48049 = t7204 * t46438;
    let t48054 = -F::new(0.3192344991997337955e-4) * t48027 + F::new(0.3192344991997337955e-4) * t48029 + F::new(0.1064114997332445985e-4) * t48031 - F::new(0.1064114997332445985e-4) * t48036 - F::new(0.42564599893297839398e-5) * t48039 - F::new(0.25538759935978703639e-4) * t48041 + F::new(0.25538759935978703639e-4) * t48043 - F::new(0.11974241701863808564e0) * t739 * t46563 - F::new(0.40911992481368012592e-1) * t48047 - F::new(0.10227998120342003148e-1) * t48049 - F::new(0.38422568777328955684e-2) * t42234 + t42239 + t42243 + t42247 + F::new(0.72042316457491791906e-3) * t42248 - F::new(0.72042316457491791906e-3) * t42250 - t42259 - t37018;
    t48054
}

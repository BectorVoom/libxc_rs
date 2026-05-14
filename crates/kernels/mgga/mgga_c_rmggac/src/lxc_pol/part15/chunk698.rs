//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 698/963 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk698<F: Float>(t2103: F, t35925: F, t25518: F, t27: F, t25640: F, t25636: F, t25525: F, t344: F, t3899: F, t35875: F, t793: F, t35924: F, t797: F, t262: F, t661: F, t851: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t36096 = t2103 * t35925;
    let t36103 = t25518 * t27;
    let t36107 = t25640 * t27;
    let t36110 = t25636 * t27;
    let t36119 = t25525 * t27;
    let t36156 = t344 * t3899;
    let t36157 = 0.30289299735990067054e-2 * t36156;
    let t36166 = t793 * t35875;
    let t36168 = t797 * t35924;
    let t36172 = t262 * t3899;
    let t36173 = t661 * t36172;
    let t36174 = 0.68992293843088486071e-3 * t36173;
    let t36188 = t851 * t35875;
    (t36096, t36103, t36107, t36110, t36119, t36157, t36166, t36168, t36172, t36174, t36188)
}

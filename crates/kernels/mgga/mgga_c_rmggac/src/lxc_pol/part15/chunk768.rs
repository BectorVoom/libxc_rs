//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 768/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk768<F: Float>(t36156: F, t35875: F, t793: F, t35924: F, t797: F, t262: F, t3899: F, t661: F, t851: F, t854: F, t305: F, t655: F) -> (F, F, F, F, F, F, F, F) {
    let t36157 = F::new(0.30289299735990067054e-2) * t36156;
    let t36166 = t793 * t35875;
    let t36168 = t797 * t35924;
    let t36172 = t262 * t3899;
    let t36173 = t661 * t36172;
    let t36174 = F::new(0.68992293843088486071e-3) * t36173;
    let t36188 = t851 * t35875;
    let t36190 = t854 * t35924;
    let t36200 = t305 * t3899;
    let t36201 = F::new(0.22765842247987981715e0) * t36200;
    let t36204 = t655 * t36172;
    (t36157, t36166, t36168, t36174, t36188, t36190, t36201, t36204)
}

//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 776/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk776<F: Float>(t344: F, t3899: F, t265: F, t5245: F, t35863: F, t797: F, t35875: F, t793: F, t35924: F, t262: F, t661: F, t854: F) -> (F, F, F, F, F, F, F, F) {
    let t36156 = t344 * t3899;
    let t36157 = F::new(0.30289299735990067054e-2) * t36156;
    let t36158 = t5245 * t265;
    let t36160 = t797 * t35863;
    let t36166 = t793 * t35875;
    let t36168 = t797 * t35924;
    let t36172 = t262 * t3899;
    let t36173 = t661 * t36172;
    let t36174 = F::new(0.68992293843088486071e-3) * t36173;
    let t36175 = t854 * t35863;
    (t36157, t36158, t36160, t36166, t36168, t36172, t36174, t36175)
}

//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 264/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk264<F: Float>(t1228: F, t492: F, t140: F, t453: F, t1144: F, t490: F, t489: F, t998: F, t1181: F, t1186: F, t1191: F, t1195: F, t1198: F, t1215: F, t1219: F, t1227: F, t467: F, t488: F) -> (F, F, F, F, F) {
    let t1229 = t1228 * t492;
    let t1231 = t453 * t140;
    let t1233 = t1231 * t490 * t1144;
    let t1237 = t489 * t490 * t998;
    let t1240 = F::new(0.54879112805223954488e-1) * t1181 * t1186 + F::new(0.12805126321218922714e0) * t1191 + F::new(0.10975822561044790898e0) * t1195 * t1198 - F::new(0.27439556402611977244e-1) * t467 * t1215 - F::new(0.27439556402611977244e-1) * t467 * t1219 + t1227 + F::new(0.25610252642437845428e0) * t1229 + F::new(0.16463733841567186346e0) * t488 * t1233 - F::new(0.54879112805223954488e-1) * t488 * t1237;
    (t1229, t1231, t1233, t1237, t1240)
}

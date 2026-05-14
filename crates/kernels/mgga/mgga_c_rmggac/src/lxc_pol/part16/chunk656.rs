//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 656/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk656<F: Float>(t10252: F, t1550: F, t9732: F, t9737: F, t1756: F, t2211: F, t1356: F, t570: F, t9530: F, t9740: F, t1707: F, t699: F, t903: F, t6522: F, t739: F, t9748: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t10253 = t1550 * t10252;
    let t10254 = 0.11974241701863808564e0 * t10253;
    let t10255 = 0.85129199786595678799e-5 * t9732;
    let t10256 = 0.1702583995731913576e-4 * t9737;
    let t10257 = t2211 * t1756;
    let t10258 = t1356 * t10257;
    let t10259 = 0.39914139006212695214e-1 * t10258;
    let t10260 = t9530 * t570;
    let t10261 = t1356 * t10260;
    let t10262 = 0.79828278012425390428e-1 * t10261;
    let t10263 = 0.17961362552795712846e0 * t9740;
    let t10267 = t699 * t1707;
    let t10268 = t903 * t10267;
    let t10269 = 0.35922725105591425692e0 * t10268;
    let t10270 = t2211 * t6522;
    let t10271 = t739 * t10270;
    let t10272 = 0.23948483403727617128e0 * t10271;
    let t10273 = 0.30487649791575028312e-3 * t9748;
    (t10254, t10255, t10256, t10257, t10259, t10260, t10262, t10263, t10267, t10269, t10270, t10272, t10273)
}

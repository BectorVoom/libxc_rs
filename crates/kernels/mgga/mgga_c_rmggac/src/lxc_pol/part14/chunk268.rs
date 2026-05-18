//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 268/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk268<F: Float>(t128: F, t794: F, t793: F, t797: F, t904: F, t305: F, t909: F, t833: F, t839: F, t838: F, t326: F, t886: F) -> (F, F, F, F, F, F) {
    let t1252 = t128 * t794;
    let t1253 = t793 * t1252;
    let t1255 = t797 * t904;
    let t1257 = t305 * t909;
    let t1259 = t128 * t833;
    let t1260 = t305 * t1259;
    let t1262 = t128 * t839;
    let t1263 = t838 * t1262;
    let t1265 = t326 * t886;
    (t1253, t1255, t1257, t1260, t1263, t1265)
}

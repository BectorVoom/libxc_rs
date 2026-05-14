//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 667/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk667<F: Float>(t10098: F, t10103: F, t10107: F, t1916: F, t708: F, t10113: F, t117: F, t5011: F, t10112: F, t6349: F, t2000: F, t326: F, t1985: F, t797: F, t838: F, t1343: F, t2048: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t10500 = 0.1702583995731913576e-4 * t10098;
    let t10501 = 0.638468998399467591e-4 * t10103;
    let t10502 = 0.15323255961587222184e-3 * t10107;
    let t10505 = t1916 * t708;
    let t10506 = 0.19957069503106347607e-1 * t10505;
    let t10507 = 0.5987120850931904282e-1 * t10113;
    let t11905 = t5011 * t117;
    let t12970 = t10112 * t117;
    let t13283 = t6349 * t117;
    let t14237 = t2000 * t326;
    let t14243 = t1985 * t797;
    let t14249 = t1985 * t838;
    let t14267 = t2048 * t1343;
    (t10500, t10501, t10502, t10506, t10507, t11905, t12970, t13283, t14237, t14243, t14249, t14267)
}

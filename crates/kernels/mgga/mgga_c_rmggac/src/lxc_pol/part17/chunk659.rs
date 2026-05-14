//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 659/964 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk659<F: Float>(t9040: F, t9060: F, t9062: F, t9075: F, t9079: F, t9091: F, t117: F, t5011: F, t10112: F, t6349: F, t2000: F, t326: F, t1985: F, t797: F, t838: F, t1343: F, t2048: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t10384 = 0.39726959900411316772e-4 * t9040;
    let t10385 = 0.47896966807455234256e0 * t9060;
    let t10386 = 0.3193131120497015617e0 * t9062;
    let t10487 = 0.15965655602485078085e0 * t9075;
    let t10496 = 0.15965655602485078085e0 * t9079;
    let t10504 = 0.39726959900411316772e-4 * t9091;
    let t11905 = t5011 * t117;
    let t12970 = t10112 * t117;
    let t13283 = t6349 * t117;
    let t14237 = t2000 * t326;
    let t14243 = t1985 * t797;
    let t14249 = t1985 * t838;
    let t14267 = t2048 * t1343;
    (t10384, t10385, t10386, t10487, t10496, t10504, t11905, t12970, t13283, t14237, t14243, t14249, t14267)
}

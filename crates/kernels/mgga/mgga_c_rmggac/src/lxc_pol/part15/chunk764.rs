//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 764/963 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk764<F: Float>(t42238: F, t357: F, t577: F, t7933: F, t7934: F, t132: F, t1412: F, t36912: F, t9082: F, t36935: F, t2185: F, t678: F, t9086: F, t8825: F, t8852: F, t8856: F) -> (F, F, F, F, F, F, F, F, F) {
    let t42239 = 0.72042316457491791906e-3 * t42238;
    let t42242 = t7933 * t7934 * t577 * t357;
    let t42243 = 0.72042316457491791906e-3 * t42242;
    let t42246 = t7933 * t7934 * t1412 * t132;
    let t42247 = 0.72042316457491791906e-3 * t42246;
    let t42248 = t36912 * t9082;
    let t42250 = t36935 * t9082;
    let t42258 = t9086 * t2185 * t678;
    let t42259 = 0.19863479950205658386e-4 * t42258;
    let t42282 = 0.11974241701863808564e0 * t8825;
    let t42289 = 0.30487649791575028314e-3 * t8852;
    let t42290 = 0.30487649791575028314e-3 * t8856;
    (t42239, t42243, t42247, t42248, t42250, t42259, t42282, t42289, t42290)
}

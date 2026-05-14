//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 886/988 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk886<F: Float>(t357: F, t577: F, t7933: F, t7934: F, t132: F, t1412: F, t36912: F, t9082: F, t36935: F, t202: F, t461: F, t5527: F, t674: F, t678: F, t2185: F, t9086: F) -> (F, F, F, F, F, F) {
    let t42242 = t7933 * t7934 * t577 * t357;
    let t42246 = t7933 * t7934 * t1412 * t132;
    let t42248 = t36912 * t9082;
    let t42250 = t36935 * t9082;
    let t42255 = t5527 * t202 * t461 * t674 * t678;
    let t42258 = t9086 * t2185 * t678;
    (t42242, t42246, t42248, t42250, t42255, t42258)
}

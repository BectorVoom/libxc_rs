//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 766/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk766<F: Float>(t303: F, t577: F, t7933: F, t7934: F, t357: F, t132: F, t1412: F, t36912: F, t9082: F, t36935: F, t2185: F, t678: F, t9086: F, t8340: F, t8344: F, t8347: F) -> (F, F, F, F, F, F, F, F, F) {
    let t42238 = t7933 * t7934 * t577 * t303;
    let t42242 = t7933 * t7934 * t577 * t357;
    let t42246 = t7933 * t7934 * t1412 * t132;
    let t42248 = t36912 * t9082;
    let t42250 = t36935 * t9082;
    let t42258 = t9086 * t2185 * t678;
    let t42369 = 0.13637330827122670865e-1 * t8340;
    let t42372 = 0.1440846329149835838e-2 * t8344;
    let t42373 = 0.1440846329149835838e-2 * t8347;
    (t42238, t42242, t42246, t42248, t42250, t42258, t42369, t42372, t42373)
}

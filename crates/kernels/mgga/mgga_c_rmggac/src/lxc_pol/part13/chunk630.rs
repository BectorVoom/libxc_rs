//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 630/988 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk630<F: Float>(t1550: F, t9000: F, t2295: F, t5016: F, t2034: F, t6355: F, t1679: F, t2157: F, t739: F, t8997: F, t132: F, t577: F, t7934: F, t7933: F, t1392: F, t202: F) -> (F, F, F, F, F, F, F, F, F) {
    let t9062 = t1550 * t9000;
    let t9071 = t5016 * t2295;
    let t9073 = t6355 * t2034;
    let t9075 = t1679 * t2157;
    let t9079 = t739 * t8997;
    let t9081 = t577 * t132;
    let t9082 = t7934 * t9081;
    let t9083 = t7933 * t9082;
    let t9085 = t1392 * t202;
    (t9062, t9071, t9073, t9075, t9079, t9081, t9082, t9083, t9085)
}

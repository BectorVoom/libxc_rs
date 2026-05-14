//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 802/963 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk802<F: Float>(t36471: F, t656: F, t6583: F, t36634: F, t6586: F, t34944: F, t6558: F, t34738: F, t6561: F, t6564: F, t34938: F, t6523: F, t8526: F, t8659: F, t2085: F, t9762: F) -> (F, F, F, F, F, F, F, F) {
    let t45120 = t36471 * t656 * t6583;
    let t45123 = t36634 * t656 * t6586;
    let t45126 = t34944 * t656 * t6558;
    let t45129 = t34738 * t656 * t6561;
    let t45132 = t36471 * t656 * t6564;
    let t45135 = t34938 * t656 * t6523;
    let t45139 = t8659 * t8526;
    let t45149 = t9762 * t2085;
    (t45120, t45123, t45126, t45129, t45132, t45135, t45139, t45149)
}

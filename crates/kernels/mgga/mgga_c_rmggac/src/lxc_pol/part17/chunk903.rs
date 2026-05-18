//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 903/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk903<F: Float>(t8526: F, t8659: F, t2085: F, t9762: F, t2010: F, t38835: F, t8465: F, t2415: F, t38820: F, t7349: F, t2329: F, t38973: F) -> (F, F, F, F, F) {
    let t45139 = t8659 * t8526;
    let t45149 = t9762 * t2085;
    let t45152 = t2010 * t8465 * t38835;
    let t45155 = t7349 * t2415 * t38820;
    let t45158 = t38973 * t2329;
    (t45139, t45149, t45152, t45155, t45158)
}

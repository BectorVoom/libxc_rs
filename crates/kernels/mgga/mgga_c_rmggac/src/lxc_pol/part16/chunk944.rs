//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 944/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk944<F: Float>(t2010: F, t45707: F, t7756: F, t7349: F, t7760: F, t9719: F, t1587: F, t2347: F, t262: F, t8640: F, t34724: F, t9709: F) -> (F, F, F, F, F, F) {
    let t45709 = t2010 * t45707 * t7756;
    let t45716 = t7349 * t9719 * t7760;
    let t45720 = t2347 * t1587;
    let t45721 = t262 * t45720;
    let t45722 = t8640 * t45721;
    let t45724 = t34724 * t9709;
    (t45709, t45716, t45720, t45721, t45722, t45724)
}

//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 967/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk967<F: Float>(t170: F, t50: F, t65: F, t16200: F, t16202: F, t16205: F, t16208: F, t16210: F, t16215: F, t16217: F, t16219: F, t16221: F, t534: F, t541: F, t555: F, t137: F, t1835: F) -> (F, F, F, F) {
    let t16224 = t65 * t50 * t170;
    let t16226 = -0.28769444444444444444e1 * t16200 + 0.27618666666666666667e2 * t16202 - 0.10229135802469135803e2 * t16205 + 0.89504938271604938273e1 * t16208 + 0.31310740740740740741e1 * t16210 + 0.366775e-1 * t16215 - 0.58684e0 * t16217 + 0.65204444444444444445e0 * t16219 + 0.5705388888888888889e0 * t16221 + 0.13490888888888888889e1 * t16224;
    let t16230 = 0.5848223622634646207e0 * t555 * t534 * t16226 * t541;
    let t16232 = 1.0 / t137 / t1835;
    (t16224, t16226, t16230, t16232)
}

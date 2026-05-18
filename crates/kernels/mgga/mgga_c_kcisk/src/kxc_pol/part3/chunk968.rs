//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 968/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk968<F: Float>(t1455: F, t4169: F, t4171: F, t4165: F, t4321: F, t1457: F, t475: F, t1520: F, t4170: F, t13369: F, t6322: F, t4230: F) -> (F, F, F, F, F) {
    let t14287 = t1455 * t4169;
    let t14289 = F::new(6.0) * t14287 * t4171;
    let t14291 = F::new(3.0) * t4165 * t4321;
    let t14292 = t1457 * t1457;
    let t14293 = F::new(1.0) / t14292;
    let t14294 = t475 * t14293;
    let t14295 = t4171 * t1520;
    let t14297 = F::new(6.0) * t14294 * t14295;
    let t14298 = t1520 * t4321;
    let t14300 = F::new(6.0) * t4170 * t14298;
    let t14301 = t6322 * t13369;
    let t14302 = t4230 * t14301;
    (t14289, t14291, t14297, t14300, t14302)
}

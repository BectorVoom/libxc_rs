//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 992/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk992<F: Float>(t14343: F, t14396: F, t14554: F, t14598: F, t1459: F, t4513: F, t9517: F, t1555: F, t524: F, t1596: F, t4348: F, t4349: F, t544: F) -> (F, F, F, F, F) {
    let t14600 = t14343 + t14396 + t14554 + t14598;
    let t14601 = t1459 * t14600;
    let t14602 = t9517 * t4513;
    let t14607 = t1555 * t1555;
    let t14608 = F::cast_from(1.0_f64) / t14607;
    let t14609 = t524 * t14608;
    let t14610 = t4348 * t1596;
    let t14612 = F::cast_from(1.0_f64) / t4349 / t544;
    (t14601, t14602, t14609, t14610, t14612)
}

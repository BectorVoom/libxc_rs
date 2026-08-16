//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 998/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk998<F: Float>(t35024: F, t8451: F, t36772: F, t8457: F, t35554: F, t8571: F, t1970: F, t1971: F, t209: F, t40427: F, t515: F, t40884: F, t7204: F) -> (F, F, F, F, F) {
    let t41893 = t8451 * t35024;
    let t41895 = t36772 * t8457;
    let t41897 = t8571 * t35554;
    let t41902 = t1970 * t1971 * t515 * t40427 * t209;
    let t41906 = t7204 * t40884;
    (t41893, t41895, t41897, t41902, t41906)
}

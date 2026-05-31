//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 404/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk404<F: Float>(t1672: F, t198: F, t185: F, t579: F, t583: F, t562: F, t181: F, t184: F) -> (F, F, F, F, F, F) {
    let t1673 = t1672 * t198;
    let t1675 = F::cast_from(4.0_f64) / F::cast_from(135.0_f64) * t185 * t1673;
    let t1676 = t579 * t583;
    let t1677 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t1676;
    let t1678 = t562 * t562;
    let t1679 = t1678 * t181;
    let t1680 = t1679 * t184;
    (t1673, t1675, t1677, t1678, t1679, t1680)
}

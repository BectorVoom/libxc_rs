//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1154/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1154<F: Float>(t184: F, t203: F, t221: F, t48341: F, t48354: F, t1044: F, t12505: F, t1815: F, t639: F, t12350: F, t1620: F, t1809: F, t7452: F) -> (F, F, F) {
    let t48359 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t203 * (t48341 + t48354) * t184 * t221;
    let t48363 = F::cast_from(32.0_f64) / F::cast_from(15.0_f64) * t639 * t1815 * t12505 * t1044;
    let t48367 = F::cast_from(64.0_f64) / F::cast_from(15.0_f64) * t1620 * t1809 * t7452 * t12350;
    (t48359, t48363, t48367)
}

//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1105/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1105<F: Float>(t12544: F, t30876: F, t12583: F, t1620: F, t1621: F, t25081: F, t12532: F, t7527: F, t3454: F, t16532: F, t185: F, t186: F) -> (F, F, F, F) {
    let t47695 = F::cast_from(16.0_f64) / F::cast_from(5.0_f64) * t30876 * t12544;
    let t47699 = F::cast_from(32.0_f64) / F::cast_from(5.0_f64) * t1620 * t1621 * t25081 * t12583;
    let t47701 = F::cast_from(32.0_f64) / F::cast_from(5.0_f64) * t7527 * t12532;
    let t47702 = t3454 * t3454;
    let t47706 = F::cast_from(16.0_f64) / F::cast_from(5.0_f64) * t185 * t186 * t16532 * t47702;
    (t47695, t47699, t47701, t47706)
}

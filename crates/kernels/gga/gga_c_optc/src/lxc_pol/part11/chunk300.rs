//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 300/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk300<F: Float>(t43: F, t50: F, t1245: F, t40: F, t1244: F, t85: F, t1235: F, t607: F, t1239: F, t611: F, zeta_threshold: F) -> (F, F, F) {
    let t44 = t43 <= zeta_threshold;
    let t51 = t50 <= zeta_threshold;
    let t1246 = t40 * t1245;
    let t1248 = 0.19751789702565206229e-1 * t1244 * t85;
    let t1251 = piecewise3(t44, 0.0, 2.0 / 3.0 * t607 * t1235);
    let t1254 = piecewise3(t51, 0.0, 2.0 / 3.0 * t611 * t1239);
    let t1256 = t1251 / 2.0 + t1254 / 2.0;
    (t1246, t1248, t1256)
}

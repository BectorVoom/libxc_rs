//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1377/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1377<F: Float>(t11700: F, t1200: F, t1565: F, t16135: F, t17582: F, t17585: F, t17610: F, t27935: F, t2886: F, t36985: F, t4249: F, t47331: F, t485: F, t53612: F, t5458: F, t5469: F, t58369: F, t58394: F, t58433: F, t58448: F, t58464: F, t58470: F, t58487: F, t58498: F, t58511: F, t58524: F, t9304: F) -> F {
    let t58528 = (t58369 + t58394 + t58433 + t58448) * t485 - F::cast_from(4.0_f64) * t53612 * t1565 + F::cast_from(12.0_f64) * t47331 * t5458 - F::cast_from(6.0_f64) * t16135 * t5469 - F::cast_from(24.0_f64) * t36985 * t17582 + F::cast_from(24.0_f64) * t11700 * t17585 - F::cast_from(4.0_f64) * t4249 * t17610 + F::cast_from(24.0_f64) * t27935 * t58464 - F::cast_from(36.0_f64) * t9304 * t5458 * t5469 + F::cast_from(6.0_f64) * t2886 * t58470 + F::cast_from(8.0_f64) * t2886 * t1565 * t17610 - t1200 * (t58487 + t58498 + t58511 + t58524);
    t58528
}

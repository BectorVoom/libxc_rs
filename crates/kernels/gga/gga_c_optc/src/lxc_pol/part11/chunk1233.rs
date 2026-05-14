//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1233/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1233<F: Float>(t43503: F, t43508: F, t44329: F, t52446: F, t52452: F, t52591: F, t52593: F, t52596: F, t52601: F, t52687: F, t52689: F, t58435: F, t11700: F, t1200: F, t1565: F, t16135: F, t17582: F, t17585: F, t17610: F, t27935: F, t2886: F, t36985: F, t4249: F, t47331: F, t485: F, t53612: F, t5458: F, t5469: F, t58369: F, t58394: F, t58433: F, t58448: F, t58464: F, t58470: F, t58487: F, t58498: F, t58511: F, t9304: F) -> (F,) {
    let t58524 = -0.72691666666666666667e3 * t58435 + 0.932345679012345679e2 * t52591 - 0.41955555555555555556e3 * t52593 + 0.12586666666666666667e4 * t52596 + 0.20977777777777777778e3 * t52601 + 0.19384444444444444445e4 * t52446 - 0.58153333333333333333e4 * t52452 - 0.19384444444444444445e4 * t43503 + 0.38768888888888888889e4 * t43508 - 0.52444444444444444446e3 * t44329 + 0.20977777777777777778e3 * t52687 - 0.12586666666666666667e4 * t52689;
    let t58528 = (t58369 + t58394 + t58433 + t58448) * t485 - 4.0 * t53612 * t1565 + 12.0 * t47331 * t5458 - 6.0 * t16135 * t5469 - 24.0 * t36985 * t17582 + 24.0 * t11700 * t17585 - 4.0 * t4249 * t17610 + 24.0 * t27935 * t58464 - 36.0 * t9304 * t5458 * t5469 + 6.0 * t2886 * t58470 + 8.0 * t2886 * t1565 * t17610 - t1200 * (t58487 + t58498 + t58511 + t58524);
    (t58528,)
}

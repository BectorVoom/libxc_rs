//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 747/1468 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk747<F: Float>(t7029: F, t7030: F, t4786: F, t586: F, t1323: F, t161: F, t165: F, t912: F, t2488: F, t6895: F, t2487: F, t1392: F, t2344: F) -> (F, F, F, F, F, F) {
    let t7031 = t7029 * t7030;
    let t7033 = t4786 * t586;
    let t7035 = t161 * t165 * t1323;
    let t7036 = t912 * t7035;
    let t7037 = t7033 * t7036;
    let t7039 = t2488 * t6895;
    let t7040 = t2487 * t7039;
    let t7042 = t1392 * t2344;
    (t7031, t7033, t7035, t7037, t7040, t7042)
}

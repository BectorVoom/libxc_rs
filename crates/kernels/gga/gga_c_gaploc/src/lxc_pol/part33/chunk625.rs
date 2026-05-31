//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 625/1464 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk625<F: Float>(t2366: F, t4324: F, t1375: F, t501: F, t1381: F, t498: F, t500: F, t177: F) -> (F, F, F, F, F, F) {
    let t4325 = t2366 * t4324;
    let t4339 = t1375 * t501;
    let t4342 = t498 * t1381;
    let t4347 = t500 * t500;
    let t4348 = F::cast_from(1.0_f64) / t4347;
    let t4349 = t177 * t4348;
    (t4325, t4339, t4342, t4347, t4348, t4349)
}

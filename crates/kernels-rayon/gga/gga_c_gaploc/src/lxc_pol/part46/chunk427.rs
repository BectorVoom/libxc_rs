//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 427/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk427(t1381: f64, t498: f64, t500: f64, t177: f64) -> (f64, f64, f64, f64) {
    let t4342 = t498 * t1381;
    let t4347 = t500 * t500;
    let t4348 = 1.0_f64 / t4347;
    let t4349 = t177 * t4348;
    (t4342, t4347, t4348, t4349)
}

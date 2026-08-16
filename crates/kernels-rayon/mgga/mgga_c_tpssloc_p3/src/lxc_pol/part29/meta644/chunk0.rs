//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2122/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2122(t2631: f64, t47285: f64, t6605: f64, t9972: f64, t12971: f64, t1894: f64, t236: f64, t6591: f64, t23046: f64, t4184: f64, t812: f64, t836: f64) -> (f64, f64, f64) {
    let t87355 = t6605 * t9972 * t47285 * t2631;
    let t87359 = t6591 * t1894 * t236 * t12971;
    let t87363 = t812 * t23046 * t836 * t4184;
    (t87355, t87359, t87363)
}

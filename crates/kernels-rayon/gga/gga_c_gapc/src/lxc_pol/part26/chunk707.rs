//! GGA_C_GAPC lxc pol — lxc_pol part 26 (v4rho2sigma2_5) CSE chunk 707/1308 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part26_v4rho2sigma2_5_chunk707(t132: f64, t2878: f64, t2881: f64, t2899: f64, t385: f64, t1510: f64, t2880: f64, t120: f64, t3954: f64, t436: f64, t2941: f64, t101: f64, t1762: f64) -> (f64, f64, f64, f64, f64) {
    let t8368 = t132 * t2878;
    let t8369 = t8368 * t2881;
    let t8371 = t385 * t2899;
    let t8373 = t2880 * t1510;
    let t8374 = t120 * t8373;
    let t8376 = t436 * t3954;
    let t8377 = t2941 * t8376;
    let t8379 = t1762 * t101;
    (t8369, t8371, t8374, t8377, t8379)
}

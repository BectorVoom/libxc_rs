//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 542/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk542(t22532: f64, t6032: f64, t3771: f64, t696: f64, t70: f64, t6034: f64, t6037: f64, t1410: f64, t236: f64, t1443: f64, t2567: f64) -> (f64, f64, f64, f64, f64) {
    let t24371 = t6032 * t22532;
    let t24372 = t3771 * t24371;
    let t24378 = t696 * t70;
    let t24380 = t6034 * t24378 * t6037;
    let t24389 = t236 * t1410;
    let t24412 = t1443 * t2567;
    (t24372, t24378, t24380, t24389, t24412)
}

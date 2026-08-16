//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1029/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1029(t34406: f64, t376: f64, t5665: f64, t32063: f64, t34380: f64, t7238: f64, t144853: f64, t1564: f64, t446: f64, t144822: f64, t7793: f64, t34482: f64, t432: f64) -> (f64, f64, f64, f64, f64) {
    let t144946 = t5665 * t376 * t34406;
    let t144950 = t7238 * t32063 * t34380;
    let t144953 = t446 * t1564 * t144853;
    let t144956 = t446 * t7793 * t144822;
    let t144958 = t34482 * t432;
    (t144946, t144950, t144953, t144956, t144958)
}

//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 661/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk661(t26533: f64, t5778: f64, t28: f64, t165: f64, t3408: f64, t23925: f64, t6587: f64, t1058: f64, t558: f64, t614: f64, t6616: f64, t376: f64, t6621: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t26534 = t5778 * t26533;
    let t26535 = t28 * t26534;
    let t26538 = t165 * t3408;
    let t26539 = t5778 * t26538;
    let t26540 = t28 * t26539;
    let t26545 = t23925 * t6587;
    let t26546 = t28 * t26545;
    let t26551 = t1058 * t558;
    let t26552 = t5778 * t26551;
    let t26553 = t28 * t26552;
    let t26560 = t6616 * t614;
    let t26561 = t28 * t26560;
    let t26564 = t376 * t6621;
    (t26535, t26538, t26540, t26546, t26551, t26553, t26561, t26564)
}

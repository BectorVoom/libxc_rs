//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 1318/1447 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk1318(t15884: f64, t3238: f64, t11683: f64, t11687: f64, t23343: f64, t11675: f64, t24195: f64, t11270: f64, t268: f64, t190: f64, t23608: f64, t24110: f64) -> (f64, f64, f64, f64, f64) {
    let t35722 = t3238 * t15884;
    let t35725 = t11687 * t11683 * t23343;
    let t35727 = t11675 * t24195;
    let t35729 = t11270 * t268;
    let t35732 = t35729 * t23608 * t190 * t24110;
    (t35722, t35725, t35727, t35729, t35732)
}

//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 283/1468 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk283(t27: f64, t338: f64, t13: f64, t355: f64, t356: f64) -> (f64, f64) {
    let t1132 = t338 * t27;
    let t1133 = 1.0_f64 / t1132;
    let t1134 = t13 * t1133;
    let t1135 = t355 * t355;
    let t1136 = t1135 * t356;
    let t1138 = 2.0_f64 * t1134 * t1136;
    (t1135, t1138)
}

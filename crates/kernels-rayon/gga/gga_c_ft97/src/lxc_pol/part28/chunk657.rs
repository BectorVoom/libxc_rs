//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 657/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk657(t26062: f64, t83: f64, t26059: f64, t1871: f64, t499: f64, t6469: f64, t110: f64, t25996: f64, t103: f64, t26041: f64, t82: f64, t26114: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t26461 = t83 * t26062;
    let t26464 = t83 * t26059;
    let t26468 = t1871 * t499 * t6469;
    let t26472 = t1871 * t110 * t25996;
    let t26476 = t82 * t26041 * t103;
    let t26480 = t83 * t26114;
    (t26461, t26464, t26468, t26472, t26476, t26480)
}

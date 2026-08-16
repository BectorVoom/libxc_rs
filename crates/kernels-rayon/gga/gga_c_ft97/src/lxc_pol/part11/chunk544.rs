//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 544/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk544(t7750: f64, t7751: f64, t27: f64, t89: f64, t1586: f64, t432: f64, t1755: f64, t28: f64, t174: f64, t358: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7752 = t7750 * t7751;
    let t7754 = t89 * t27 * t7752;
    let t7755 = t1586 * t432;
    let t7756 = t7755 * t1755;
    let t7758 = t89 * t28 * t7756;
    let t7760 = 1.0_f64 / t174 / t358;
    (t7752, t7754, t7755, t7756, t7758, t7760)
}

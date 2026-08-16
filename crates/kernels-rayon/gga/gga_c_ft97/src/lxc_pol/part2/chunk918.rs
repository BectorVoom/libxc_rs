//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 918/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk918(t1882: f64, t3983: f64, t1131: f64, t2459: f64, t2574: f64, t265: f64, t3746: f64, t724: f64, t773: f64, t3839: f64, t1140: f64, t8232: f64) -> (f64, f64, f64, f64, f64) {
    let t14212 = 2.0_f64 / 9.0_f64 * t1882 * t3983;
    let t14213 = t1131 * t2459;
    let t14215 = t2574 * t265 * t14213;
    let t14219 = t724 * t773 * t3746;
    let t14223 = 4.0_f64 / 9.0_f64 * t1882 * t3839;
    let t14224 = t8232 * t1140;
    (t14212, t14215, t14219, t14223, t14224)
}

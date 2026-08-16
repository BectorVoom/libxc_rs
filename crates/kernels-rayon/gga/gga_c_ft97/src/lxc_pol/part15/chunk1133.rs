//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1133/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1133(t21251: f64, t215: f64, t4960: f64, t5005: f64, t1127: f64, t5049: f64, t207: f64, t35382: f64, t1690: f64, t5010: f64, t21172: f64, t38176: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t88880 = t21251 * t215;
    let t88881 = 1.0_f64 / t88880;
    let t88891 = t4960 * t5005;
    let t88898 = t5049 * t1127;
    let t88909 = 1.0_f64 / t207 / t35382;
    let t88911 = t1690 * t5010 * t88909;
    let t88916 = t38176 * t21172;
    (t88881, t88891, t88898, t88909, t88911, t88916)
}

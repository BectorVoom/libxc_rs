//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 601/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk601(t215: f64, t207: f64, t1690: f64, t5010: f64, t1127: f64, t3780: f64, t2427: f64, t14: f64, t4995: f64, t228: f64, t231: f64, t1124: f64, t3799: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t5011 = t215 * t215;
    let t5014 = 1.0_f64 / t207 / t5011 / t215;
    let t5016 = t1690 * t5010 * t5014;
    let t5019 = t3780 * t1127;
    let t5025 = t1127 * t1127;
    let t5026 = t2427 * t5025;
    let t5029 = t4995 * t14;
    let t5031 = t228 * t5029 * t231;
    let t5034 = t3799 * t1124;
    (t5014, t5016, t5019, t5025, t5026, t5031, t5034)
}

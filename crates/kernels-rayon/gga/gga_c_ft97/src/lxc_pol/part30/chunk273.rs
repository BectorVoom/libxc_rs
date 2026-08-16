//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 273/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk273(t1127: f64, t2426: f64, t709: f64, t1103: f64, t172: f64, t228: f64, t231: f64, t227: f64, t9: f64, t706: f64, t1123: f64, t173: f64) -> (f64, f64, f64, f64) {
    let t3790 = t2426 * t1127;
    let t3791 = t3790 * t709;
    let t3794 = t1103 * t172;
    let t3796 = t228 * t3794 * t231;
    let t3799 = t9 * t227 * t1103;
    let t3800 = t3799 * t706;
    let t3803 = t173 * t1123;
    (t3791, t3796, t3800, t3803)
}

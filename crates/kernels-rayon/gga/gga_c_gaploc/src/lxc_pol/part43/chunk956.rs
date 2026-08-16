//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 956/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk956(t13756: f64, t380: f64, t12035: f64, t6556: f64, t39340: f64, t921: f64, t12032: f64, t2497: f64, t12148: f64, t1382: f64, t13838: f64, t5559: f64, t841: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t47054 = 0.37940008847568199465e-1_f64 * t380 * t13756;
    let t47064 = t6556 * t12035;
    let t47071 = t39340 * t921;
    let t47075 = t12032 * t2497;
    let t47077 = t1382 * t12148 * t921;
    let t47080 = t5559 * t13838 * t841;
    (t47054, t47064, t47071, t47075, t47077, t47080)
}

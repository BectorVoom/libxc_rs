//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 653/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk653(t173: f64, t5037: f64, t701: f64, t5041: f64, t3799: f64, t3803: f64, t227: f64, t4995: f64, t9: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t18034 = t173 * t5037;
    let t18035 = t701 * t18034;
    let t18037 = t173 * t5041;
    let t18038 = t701 * t18037;
    let t18040 = t3799 * t3803;
    let t18043 = t9 * t227 * t4995;
    (t18034, t18035, t18037, t18038, t18040, t18043)
}

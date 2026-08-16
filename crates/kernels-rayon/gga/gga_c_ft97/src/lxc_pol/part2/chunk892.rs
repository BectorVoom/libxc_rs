//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 892/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk892(t13724: f64, t13761: f64, t13796: f64, t13825: f64, t258: f64, t3951: f64, t761: f64, t766: f64, t242: f64, t1175: f64, t2459: f64, t729: f64) -> (f64, f64, f64, f64, f64) {
    let t13827 = t13724 + t13761 + t13796 + t13825;
    let t13828 = t13827 * t258;
    let t13830 = t3951 * t761;
    let t13831 = t13830 * t766;
    let t13832 = t242 * t13831;
    let t13836 = t729 * t1175 * t2459;
    (t13827, t13828, t13831, t13832, t13836)
}

//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 625/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk625(t28057: f64, t28069: f64, t28082: f64, t28095: f64, t258: f64, t2469: f64, t6940: f64, t242: f64, t27981: f64, t1882: f64, t6914: f64, t1131: f64, t6187: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t28097 = t28057 + t28069 + t28082 + t28095;
    let t28098 = t28097 * t258;
    let t28100 = t2469 * t6940;
    let t28102 = t242 * t27981;
    let t28106 = t1882 * t6914;
    let t28108 = t6187 * t1131;
    (t28097, t28098, t28100, t28102, t28106, t28108)
}

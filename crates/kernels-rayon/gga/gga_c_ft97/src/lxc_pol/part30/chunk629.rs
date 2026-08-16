//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 629/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk629(t28140: f64, t28141: f64, t24737: f64, t3842: f64, t13885: f64, t1901: f64, t24567: f64, t28102: f64, t28106: f64, t28110: f64, t28113: f64, t28116: f64, t28120: f64, t28125: f64, t28130: f64, t28133: f64, t28137: f64, t446: f64) -> (f64, f64) {
    let t28142 = t28140 * t28141;
    let t28145 = t24737 * t3842;
    let t28146 = t13885 * t28145;
    let t28149 = -t446 * t28102 / 3.0_f64 - 2.0_f64 / 9.0_f64 * t24567 + t28106 / 9.0_f64 + t446 * t28110 / 3.0_f64 - t28113 / 9.0_f64 - t446 * t28116 / 3.0_f64 - t446 * t28120 / 3.0_f64 + t1901 * t28125 / 9.0_f64 - 2.0_f64 / 3.0_f64 * t1901 * t28130 + t1901 * t28133 / 9.0_f64 - 2.0_f64 / 3.0_f64 * t1901 * t28137 - 2.0_f64 * t1901 * t28142 - 2.0_f64 / 3.0_f64 * t1901 * t28146;
    (t28145, t28149)
}

//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 640/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk640(t6837: f64, t729: f64, t773: f64, t242: f64, t27987: f64, t1901: f64, t24731: f64, t24733: f64, t24735: f64, t28286: f64, t28289: f64, t28291: f64, t28295: f64, t28302: f64, t28305: f64, t28309: f64, t28312: f64, t446: f64) -> f64 {
    let t28319 = t729 * t773 * t6837;
    let t28322 = t242 * t27987;
    let t28325 = t446 * t28286 / 3.0_f64 + t28289 / 27.0_f64 + t1901 * t28291 / 9.0_f64 - 2.0_f64 / 3.0_f64 * t1901 * t28295 - 2.0_f64 * t1901 * t28302 + t1901 * t28305 / 9.0_f64 + t1901 * t28309 / 9.0_f64 + t1901 * t28312 / 9.0_f64 + t24731 / 9.0_f64 + t24733 / 9.0_f64 + t24735 / 9.0_f64 - t446 * t28319 / 3.0_f64 + 2.0_f64 / 3.0_f64 * t446 * t28322;
    t28325
}

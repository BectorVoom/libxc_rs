//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1090/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1090(t1882: f64, t34765: f64, t34640: f64, t34779: f64, t34637: f64, t103: f64, t11854: f64, t138154: f64, t138156: f64, t138158: f64, t138168: f64, t138176: f64, t138178: f64, t138184: f64, t138208: f64, t145909: f64, t1557: f64, t1570: f64, t1901: f64, t23265: f64, t25990: f64, t26171: f64, t28: f64, t3188: f64, t32325: f64, t32366: f64, t32602: f64, t446: f64, t447: f64, t452: f64, t47399: f64, t488: f64, t5630: f64, t5743: f64, t60901: f64, t7274: f64, t82: f64, t89: f64, t920: f64, t925: f64, t979: f64) -> f64 {
    let t146824 = t1882 * t34765;
    let t146826 = t1882 * t34640;
    let t146834 = t1882 * t34779;
    let t146851 = t1882 * t34637;
    let t146858 = 2.0_f64 / 3.0_f64 * t138154 + 2.0_f64 / 9.0_f64 * t138156 - 4.0_f64 * t1901 * t26171 * t5630 * t25990 - 2.0_f64 / 9.0_f64 * t138158 + t138168 / 9.0_f64 - 4.0_f64 / 3.0_f64 * t1901 * t60901 * t32602 - 2.0_f64 / 9.0_f64 * t138176 - 2.0_f64 / 9.0_f64 * t138178 - t446 * t447 * t32366 * t925 / 9.0_f64 - 2.0_f64 / 9.0_f64 * t146824 - 4.0_f64 / 9.0_f64 * t146826 - 2.0_f64 / 9.0_f64 * t138184 + t446 * t452 * t488 * t32325 * t979 / 3.0_f64 + t138208 + t146834 / 9.0_f64 - 4.0_f64 / 9.0_f64 * t1901 * t11854 * t23265 * t920 * t5743 + 4.0_f64 / 9.0_f64 * t1901 * t11854 * t7274 * t1570 * t3188 - 4.0_f64 / 27.0_f64 * t1901 * t47399 * t7274 * t1557 * t3188 + 2.0_f64 / 3.0_f64 * t146851 + t89 * t28 * t82 * t145909 * t103 / 3.0_f64;
    t146858
}

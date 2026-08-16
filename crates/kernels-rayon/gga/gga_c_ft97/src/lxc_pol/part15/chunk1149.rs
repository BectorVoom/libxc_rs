//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1149/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1149(t5064: f64, t41409: f64, t13927: f64, t21499: f64, t10052: f64, t5147: f64, t1091: f64, t14159: f64, t18729: f64, t1901: f64, t21472: f64, t21490: f64, t21664: f64, t242: f64, t2599: f64, t3977: f64, t42334: f64, t446: f64, t4965: f64, t4969: f64, t4973: f64, t5181: f64, t724: f64, t729: f64, t81545: f64, t81547: f64, t9803: f64) -> (f64, f64, f64, f64) {
    let t89441 = t5064 * t5064;
    let t89442 = t41409 * t89441;
    let t89456 = t13927 * t21499;
    let t89465 = t10052 * t5064 * t5147;
    let t89472 = 2.0_f64 / 3.0_f64 * t1901 * t2599 * t18729 * t4973 + 4.0_f64 / 9.0_f64 * t1901 * t9803 * t18729 * t4965 + 8.0_f64 * t446 * t242 * t89442 + 4.0_f64 / 3.0_f64 * t446 * t724 * t5181 * t4969 - 4.0_f64 / 9.0_f64 * t81545 - 4.0_f64 / 9.0_f64 * t81547 + 8.0_f64 / 3.0_f64 * t1901 * t42334 * t21472 * t1091 + 8.0_f64 * t446 * t242 * t89456 + 4.0_f64 * t446 * t729 * t3977 * t21490 - 12.0_f64 * t446 * t242 * t89465 + 4.0_f64 / 3.0_f64 * t1901 * t14159 * t21664;
    (t89442, t89456, t89465, t89472)
}

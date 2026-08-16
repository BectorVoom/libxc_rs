//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1198/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1198(t22249: f64, t55768: f64, t22346: f64, t4246: f64, t1091: f64, t1255: f64, t21355: f64, t22471: f64, t2857: f64, t2862: f64, t446: f64, t4969: f64, t5225: f64, t5393: f64, t5424: f64, t56665: f64, t56689: f64, t72167: f64, t835: f64, t84486: f64, t84500: f64, t84504: f64, t84547: f64, t871: f64) -> (f64, f64, f64) {
    let t90803 = t55768 * t22249;
    let t90817 = t4246 * t22346;
    let t90863 = -8.0_f64 / 27.0_f64 * t84486 - 8.0_f64 / 3.0_f64 * t84500 + 4.0_f64 / 3.0_f64 * t446 * t835 * t5424 * t4969 + 16.0_f64 / 9.0_f64 * t446 * t2857 * t1255 * t21355 - 4.0_f64 / 9.0_f64 * t446 * t835 * t22471 * t1091 + 8.0_f64 / 3.0_f64 * t84504 - 4.0_f64 * t446 * t2862 * t871 * t5225 * t5393 + 16.0_f64 / 27.0_f64 * t72167 + 112.0_f64 / 81.0_f64 * t56665 - 112.0_f64 / 81.0_f64 * t56689 + 4.0_f64 / 3.0_f64 * t84547;
    (t90803, t90817, t90863)
}

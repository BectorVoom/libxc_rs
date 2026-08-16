//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1002/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1002(t1286: f64, t25912: f64, t1307: f64, t1308: f64, t137376: f64, t137398: f64, t137400: f64, t1526: f64, t1527: f64, t15567: f64, t2258: f64, t25905: f64, t25909: f64, t25916: f64, t25956: f64, t25969: f64, t25972: f64, t25975: f64, t25978: f64, t2984: f64, t2993: f64, t3000: f64, t3052: f64, t32033: f64, t356: f64, t5495: f64, t5501: f64, t5618: f64, t5697: f64, t6414: f64, t6517: f64, t8633: f64, t925: f64) -> f64 {
    let t144472 = t1286 * t25912;
    let t144503 = -t5501 * t25975 / 9.0_f64 + t5501 * t25978 / 27.0_f64 - t5501 * t25969 / 9.0_f64 - t5501 * t25972 / 9.0_f64 + t5495 * t6517 / 3.0_f64 + t1286 * t25916 / 3.0_f64 - t144472 / 9.0_f64 + t1286 * t25909 / 3.0_f64 + t1286 * t25905 / 3.0_f64 - t15567 * t8633 * t1307 * t2984 / 9.0_f64 - t137376 / 54.0_f64 + t137398 - t137400 / 12.0_f64 + t6414 * t5697 / 3.0_f64 + t15567 * t2258 * t1307 * t2993 / 6.0_f64 + t6414 * t32033 / 18.0_f64 + t1286 * t3000 * t1308 * t3052 / 9.0_f64 + t1286 * t356 * t5618 * t925 / 18.0_f64 - t1526 * t1527 * t25956 / 12.0_f64;
    t144503
}

//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2332/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2332(t46657: f64, t5593: f64, t120: f64, t20852: f64, t13258: f64, t20983: f64, t16839: f64, t16841: f64, t2643: f64, t4178: f64, t4180: f64, t4182: f64, t4234: f64, t47307: f64, t58353: f64, t58363: f64, t58373: f64, t58379: f64, t58381: f64, t58904: f64, t67596: f64, t67607: f64, t829: f64) -> (f64, f64) {
    let t67612 = t46657 * t5593;
    let t67620 = t120 * t20852;
    let t67625 = t13258 * t20983;
    let t67636 = -t2643 * t4180 * t16839 * t4234 / 1024.0_f64 - t2643 * t4180 * t67607 * t829 / 3072.0_f64 - 7.0_f64 / 192.0_f64 * t67612 - 3.0_f64 / 512.0_f64 * t58904 * t16841 + t47307 * t4180 * t67607 * t67596 / 128.0_f64 + t4178 * t4180 * t67620 * t4182 / 1536.0_f64 + 7.0_f64 / 192.0_f64 * t67625 - t2643 * t4180 * t67620 * t829 / 3072.0_f64 + 7.0_f64 / 256.0_f64 * t58353 + 7.0_f64 / 1536.0_f64 * t58363 - 7.0_f64 / 192.0_f64 * t58373 - 7.0_f64 / 192.0_f64 * t58379 + 7.0_f64 / 768.0_f64 * t58381;
    (t67620, t67636)
}

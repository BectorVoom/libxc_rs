//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2206/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2206(t25010: f64, t7685: f64, t16944: f64, t25014: f64, t25365: f64, t86721: f64, t22960: f64, t67128: f64, t1877: f64, t2219: f64, t7541: f64, t5527: f64, t606: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t97949 = 2.0_f64 * t7685 * t25010;
    let t97950 = t25014 * t16944;
    let t97953 = t86721 * t25365;
    let t97956 = t22960 * t67128;
    let t97972 = 2.0_f64 * t1877 * t7541 * t2219;
    let t97985 = t606 * t5527;
    (t97949, t97950, t97953, t97956, t97972, t97985)
}

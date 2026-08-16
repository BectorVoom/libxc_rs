//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1207/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1207(t3275: f64, t3277: f64, t44040: f64, t11550: f64, t12056: f64, t3262: f64, t11189: f64, t43979: f64, t3579: f64, t41327: f64, t39010: f64, t42472: f64) -> (f64, f64, f64, f64, f64) {
    let t44043 = 5.0_f64 / 16.0_f64 * t3275 * t44040 * t3277;
    let t44046 = 3.0_f64 / 2.0_f64 * t3262 * t12056 * t11550;
    let t44049 = 45.0_f64 / 32.0_f64 * t3275 * t11189 * t43979;
    let t44051 = 5.0_f64 / 8.0_f64 * t3579 * t41327;
    let t44054 = 585.0_f64 / 256.0_f64 * t3275 * t39010 * t42472;
    (t44043, t44046, t44049, t44051, t44054)
}

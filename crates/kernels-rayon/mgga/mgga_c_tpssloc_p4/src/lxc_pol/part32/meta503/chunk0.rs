//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1827/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1827(t1388: f64, t1845: f64, t26162: f64, t26161: f64, t532: f64, t7752: f64, t6879: f64, t1983: f64, t1874: f64, t26114: f64, t4072: f64, t89: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t26163 = t1845 * t1388;
    let t26164 = t26162 * t26163;
    let t26166 = 2.0_f64 * t26161 * t26164;
    let t26167 = t532 * t7752;
    let t26168 = t26167 * t6879;
    let t26170 = 3.0_f64 * t1983 * t26168;
    let t26178 = 2.0_f64 * t26114 * t1874;
    let t26179 = t89 * t4072;
    (t26163, t26164, t26166, t26167, t26168, t26170, t26178, t26179)
}

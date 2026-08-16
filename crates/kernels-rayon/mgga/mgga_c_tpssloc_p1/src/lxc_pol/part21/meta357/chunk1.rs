//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 1773/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1773(t13278: f64, t831: f64, t2629: f64, t4166: f64, t4250: f64, t9638: f64, t1495: f64, t210: f64, t2379: f64, t4158: f64, t776: f64, t2553: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13280 = 7.0_f64 / 2304.0_f64 * t13278 * t831;
    let t13283 = t4166 * t2629;
    let t13287 = 7.0_f64 / 576.0_f64 * t9638 * t4250;
    let t13289 = t210 * t1495 * t2379;
    let t13293 = t210 * t4158 * t776;
    let t13297 = t210 * t1495 * t2553;
    (t13280, t13283, t13287, t13289, t13293, t13297)
}

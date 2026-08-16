//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1749/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1749(t2639: f64, t4236: f64, t1512: f64, t9674: f64, t2638: f64, t4166: f64, t831: f64, t2629: f64, t4250: f64, t9638: f64, t1495: f64, t210: f64, t2379: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t13275 = 7.0_f64 / 2304.0_f64 * t2639 * t4236;
    let t13277 = 7.0_f64 / 2304.0_f64 * t9674 * t1512;
    let t13278 = t4166 * t2638;
    let t13280 = 7.0_f64 / 2304.0_f64 * t13278 * t831;
    let t13283 = t4166 * t2629;
    let t13287 = 7.0_f64 / 576.0_f64 * t9638 * t4250;
    let t13289 = t210 * t1495 * t2379;
    (t13275, t13277, t13278, t13280, t13283, t13287, t13289)
}

//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 688/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk688(t1762: f64, t5210: f64, t1734: f64, t1771: f64, t124: f64, t704: f64, t706: f64, t1672: f64, t584: f64, t1759: f64, t1871: f64, t616: f64) -> (f64, f64, f64, f64, f64) {
    let t5212 = 0.65061487801810439052e-1_f64 * t1762 * t5210;
    let t5213 = t1771 * t1734;
    let t5215 = t124 * t704;
    let t5216 = t5215 * t706;
    let t5218 = 0.43374325201206959369e-1_f64 * t1762 * t5216;
    let t5219 = t584 * t1672;
    let t5220 = t5219 * t1759;
    let t5222 = t616 * t1871;
    (t5212, t5213, t5218, t5220, t5222)
}

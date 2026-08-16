//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1150/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1150(t3806: f64, t5248: f64, t550: f64, t31170: f64, t553: f64, t835: f64, t544: f64, t8467: f64, t1369: f64, t8466: f64, t6883: f64, t8480: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t31172 = t5248 * t3806 * t550;
    let t31173 = t31170 * t31172;
    let t31175 = t553 * t835;
    let t31176 = t544 * t31175;
    let t31177 = t31176 * t8467;
    let t31179 = t8466 * t1369;
    let t31192 = 0.38381794893125283518e-1_f64 * t6883 * t8480;
    (t31172, t31173, t31175, t31176, t31177, t31179, t31192)
}

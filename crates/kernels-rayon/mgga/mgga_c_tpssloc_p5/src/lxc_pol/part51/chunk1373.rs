//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1373/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1373(t25992: f64, t8607: f64, t102344: f64, t1874: f64, t27188: f64, t6525: f64, t92090: f64, t33603: f64, t6876: f64, t31297: f64, t7685: f64, t114360: f64, t121181: f64, t121184: f64, t121190: f64, t121192: f64, t121194: f64, t26974: f64, t31055: f64, t8329: f64) -> f64 {
    let t121195 = t8607 * t25992;
    let t121197 = 2.0_f64 * t102344 * t1874;
    let t121199 = 2.0_f64 * t27188 * t6525;
    let t121201 = 2.0_f64 * t92090 * t1874;
    let t121203 = 3.0_f64 * t6876 * t33603;
    let t121204 = t7685 * t31297;
    let t121205 = -3.0_f64 * t114360 * t26974 - t121181 + t121184 - t121190 - t121192 - t121194 - t121195 - t121197 - t121199 - t121201 + t121203 - t121204 - t31055 - t8329;
    t121205
}

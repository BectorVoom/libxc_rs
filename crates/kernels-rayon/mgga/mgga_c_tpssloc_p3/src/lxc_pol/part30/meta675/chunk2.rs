//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2106/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2106(t27960: f64, t645: f64, t72: f64, t4021: f64, t7431: f64, t1864: f64, t5389: f64, t1863: f64, t22544: f64, t26009: f64, t26013: f64, t26016: f64, t27937: f64, t33567: f64, t6506: f64, t6510: f64, t83717: f64, t83830: f64, t90087: f64, t90091: f64, t90095: f64, t90098: f64, t90101: f64, t90104: f64, t9239: f64) -> f64 {
    let t96418 = t72 * t27960 * t645;
    let t96422 = t72 * t7431 * t4021;
    let t96425 = t1864 * t5389;
    let t96426 = t1863 * t96425;
    let t96441 = 20.0_f64 * t9239 * t33567 * t26009 - t27937 * t6506 / 6.0_f64 - t27937 * t6510 / 6.0_f64 + 35.0_f64 * t83830 * t96418 - 10.0_f64 * t22544 * t96422 + 10.0_f64 * t83717 * t96426 - 10.0_f64 / 3.0_f64 * t90098 * t26013 - 10.0_f64 / 3.0_f64 * t90101 * t26013 - 10.0_f64 / 3.0_f64 * t90104 * t26013 - 10.0_f64 / 3.0_f64 * t26016 * t90087 - 10.0_f64 / 3.0_f64 * t26016 * t90091 - 10.0_f64 / 3.0_f64 * t26016 * t90095;
    t96441
}

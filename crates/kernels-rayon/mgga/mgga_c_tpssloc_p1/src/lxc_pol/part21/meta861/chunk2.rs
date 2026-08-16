//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3124/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3124(t15419: f64, t18215: f64, t3447: f64, t18469: f64, t44525: f64, t18206: f64, t52133: f64, t15324: f64, t15327: f64, t15376: f64, t15379: f64, t15391: f64, t44529: f64, t44558: f64, t4900: f64, t63386: f64, t63394: f64) -> f64 {
    let t64624 = t3447 * t15419 * t18215;
    let t64627 = t3447 * t44525 * t18469;
    let t64632 = t3447 * t52133 * t18206;
    let t64634 = -0.2962962962962962963e-2_f64 * t15376 * t15327 + 0.22222222222222222221e-2_f64 * t3447 * t4900 * t63386 - 0.14814814814814814815e-2_f64 * t15376 * t15324 + 0.74074074074074074072e-3_f64 * t3447 * t4900 * t63394 - 0.37037037037037037036e-3_f64 * t3447 * t44529 * t18469 - 0.14814814814814814814e-2_f64 * t15376 * t15379 - 0.37037037037037037036e-3_f64 * t3447 * t44558 * t18469 + 0.49382716049382716048e-3_f64 * t64624 - 0.24691358024691358024e-3_f64 * t64627 + 0.1975308641975308642e-2_f64 * t15376 * t15391 - 0.5761316872427983539e-3_f64 * t64632;
    t64634
}

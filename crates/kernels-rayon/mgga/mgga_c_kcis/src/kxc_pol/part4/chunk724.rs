//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 724/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk724(t1455: f64, t531: f64, t1517: f64, t833: f64, t1444: f64, t538: f64, t2642: f64, t1518: f64, t2645: f64, t4106: f64, t509: f64, t1153: f64, t1478: f64, t1483: f64, t1507: f64, t368: f64, t3782: f64, t3788: f64, t3810: f64, t3816: f64, t3842: f64, t4193: f64, t4202: f64, t4213: f64, t4214: f64, t4217: f64, t4222: f64, t545: f64, t562: f64, t86: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4225 = t1455 * t531;
    let t4227 = t1517 * t4225 * t833;
    let t4230 = t538 * t1444;
    let t4232 = t1517 * t4230 * t2642;
    let t4236 = t1517 * t1518 * t2645;
    let t4239 = t509 * t4106;
    let t4243 = 0.619125e-2_f64 * t4193 * t545 + 0.1857375e-1_f64 * t1507 * t1478 - 0.123825e-1_f64 * t1507 * t1483 + 0.46434375e-2_f64 * t562 * t3782 - 0.1857375e-1_f64 * t4202 * t3788 + 0.9286875e-2_f64 * t562 * t3810 + 0.123825e-1_f64 * t562 * t3816 - 0.619125e-2_f64 * t562 * t3842 + t4213 - 0.35374814814814814814e-1_f64 * t4214 - 0.53062222222222222222e-1_f64 * t4217 - 0.44218518518518518518e-1_f64 * t1153 * t4222 - 0.53062222222222222222e-1_f64 * t1153 * t4227 + 0.53062222222222222222e-1_f64 * t1153 * t4232 - 0.26531111111111111111e-1_f64 * t1153 * t4236 - 0.39796666666666666666e-1_f64 * t86 * t368 * t4239;
    (t4225, t4227, t4232, t4236, t4239, t4243)
}

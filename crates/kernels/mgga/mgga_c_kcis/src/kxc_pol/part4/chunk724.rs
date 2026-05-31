//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 724/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk724<F: Float>(t1455: F, t531: F, t1517: F, t833: F, t1444: F, t538: F, t2642: F, t1518: F, t2645: F, t4106: F, t509: F, t1153: F, t1478: F, t1483: F, t1507: F, t368: F, t3782: F, t3788: F, t3810: F, t3816: F, t3842: F, t4193: F, t4202: F, t4213: F, t4214: F, t4217: F, t4222: F, t545: F, t562: F, t86: F) -> (F, F, F, F, F, F) {
    let t4225 = t1455 * t531;
    let t4227 = t1517 * t4225 * t833;
    let t4230 = t538 * t1444;
    let t4232 = t1517 * t4230 * t2642;
    let t4236 = t1517 * t1518 * t2645;
    let t4239 = t509 * t4106;
    let t4243 = F::cast_from(0.619125e-2_f64) * t4193 * t545 + F::cast_from(0.1857375e-1_f64) * t1507 * t1478 - F::cast_from(0.123825e-1_f64) * t1507 * t1483 + F::cast_from(0.46434375e-2_f64) * t562 * t3782 - F::cast_from(0.1857375e-1_f64) * t4202 * t3788 + F::cast_from(0.9286875e-2_f64) * t562 * t3810 + F::cast_from(0.123825e-1_f64) * t562 * t3816 - F::cast_from(0.619125e-2_f64) * t562 * t3842 + t4213 - F::cast_from(0.35374814814814814814e-1_f64) * t4214 - F::cast_from(0.53062222222222222222e-1_f64) * t4217 - F::cast_from(0.44218518518518518518e-1_f64) * t1153 * t4222 - F::cast_from(0.53062222222222222222e-1_f64) * t1153 * t4227 + F::cast_from(0.53062222222222222222e-1_f64) * t1153 * t4232 - F::cast_from(0.26531111111111111111e-1_f64) * t1153 * t4236 - F::cast_from(0.39796666666666666666e-1_f64) * t86 * t368 * t4239;
    (t4225, t4227, t4232, t4236, t4239, t4243)
}

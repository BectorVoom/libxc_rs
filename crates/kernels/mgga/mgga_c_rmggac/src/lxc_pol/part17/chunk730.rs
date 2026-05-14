//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 730/964 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk730<F: Float>(t39555: F, t1562: F, t7894: F, t623: F, t7191: F, t34884: F, t9206: F, t36924: F, t9082: F, t321: F, t8915: F, t262: F, t7204: F, t2157: F, t5011: F, t333: F, t8708: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t39556 = 0.10248087766267884742e-3 * t39555;
    let t39558 = 0.4726e1 * t1562 * t7894;
    let t39570 = t623 * t7191;
    let t39591 = t34884 * t9206;
    let t39609 = t36924 * t9082;
    let t39665 = t8915 * t321;
    let t39666 = t262 * t39665;
    let t39667 = t7204 * t39666;
    let t39678 = t5011 * t2157;
    let t39679 = 0.79828278012425390426e-1 * t39678;
    let t39692 = t8708 * t333;
    (t39556, t39558, t39570, t39591, t39609, t39665, t39666, t39667, t39679, t39692)
}

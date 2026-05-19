//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 697/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk697<F: Float>(t102: F, t6599: F, t108: F, t176: F, t203: F, t2226: F, t616: F, t2234: F, t758: F, t1986: F, t1990: F, t6342: F, t6356: F, t6526: F, t6530: F, t6563: F, t6571: F, t6572: F) -> (F, F, F, F) {
    let t6600 = t6599 * t102;
    let t6602 = t176 * t6600 * t108;
    let t6604 = t6602 * t203 / F::new(2.0);
    let t6605 = t2226 * t616;
    let t6607 = t176 * t6605 * t108;
    let t6608 = t6607 * t203;
    let t6610 = t2234 * t758;
    let t6612 = t1986 * t1990;
    let t6613 = F::cast_from(0.35089340384731224426e1_f64) * t6612;
    let t6614 = t6342 + t6526 - t6356 + F::new(3.0) / F::new(2.0) * t6530 + t6563 * t203 / F::new(2.0) - t6571 + F::new(35.0) / F::new(3.0) * t6572 + t6604 + F::new(3.0) / F::new(2.0) * t6608 + F::new(3.0) * t6610 + t6613;
    (t6602, t6607, t6613, t6614)
}

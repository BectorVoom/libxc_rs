//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1201/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1201<F: Float>(t7501: F, t824: F, t23801: F, t256: F, t7758: F, t805: F, t243: F, t2491: F, t2516: F, t23543: F, t23545: F, t23551: F, t23553: F, t23555: F, t23557: F, t23561: F, t23565: F, t23567: F, t23569: F, t23840: F, t23842: F, t23846: F, t23874: F) -> (F, F, F, F, F) {
    let t24792 = t824 * t7501;
    let t24795 = t256 * t23801;
    let t24799 = t805 * t7758;
    let t24804 = t243 / t2516 / t2491;
    let t24824 = -F::cast_from(0.23154444444444444445e0_f64) * t23543 - F::cast_from(0.55570666666666666668e0_f64) * t23545 + F::cast_from(0.55570666666666666666e0_f64) * t23551 + F::cast_from(0.12349037037037037037e1_f64) * t23553 + F::cast_from(0.94674375e0_f64) * t23840 - F::cast_from(0.52945875e1_f64) * t23842 + F::cast_from(0.2366859375e0_f64) * t23846 + F::cast_from(0.6311625e0_f64) * t23874 + F::cast_from(0.13892666666666666667e1_f64) * t23555 + F::cast_from(0.166712e1_f64) * t23557 - F::cast_from(0.125034e1_f64) * t23561 - F::cast_from(0.104195e0_f64) * t23565 + F::cast_from(0.27785333333333333333e0_f64) * t23567 + F::cast_from(0.12349037037037037037e0_f64) * t23569;
    (t24792, t24795, t24799, t24804, t24824)
}

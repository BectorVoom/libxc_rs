//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 370/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk370<F: Float>(t1767: F, t1770: F, t1773: F, t1777: F, t1779: F, t1782: F) -> F {
    let t1784 = -F::cast_from(0.42198333333333333333e0_f64) * t1767 + F::cast_from(0.84396666666666666666e0_f64) * t1770 + F::cast_from(0.39862222222222222223e0_f64) * t1773 + F::cast_from(0.68258333333333333333e-1_f64) * t1777 + F::cast_from(0.13651666666666666667e0_f64) * t1779 + F::cast_from(0.13692777777777777778e0_f64) * t1782;
    t1784
}

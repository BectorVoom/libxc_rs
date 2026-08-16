//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 1095/1303 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk1095<F: Float>(t10556: F, t10636: F, t13563: F, t13598: F, t14245: F, t14246: F, t17149: F, t17154: F, t17159: F, t17163: F, t17165: F, t17169: F, t17173: F, t17175: F, t17180: F, t17185: F, t17189: F) -> F {
    let t17488 = -t10636 - F::cast_from(0.79148148148148148147e-2_f64) * t10556 - F::cast_from(0.15829629629629629629e-1_f64) * t13598 + F::cast_from(0.79148148148148148147e-2_f64) * t13563 - t14245 + t14246 + F::cast_from(0.39574074074074074073e-2_f64) * t17149 - F::cast_from(0.19787037037037037037e-1_f64) * t17154 + F::cast_from(0.71233333333333333332e-1_f64) * t17159 - F::cast_from(0.23744444444444444444e-1_f64) * t17163 - F::cast_from(0.11872222222222222222e-1_f64) * t17165 - F::cast_from(0.10685e0_f64) * t17169 + F::cast_from(0.71233333333333333332e-1_f64) * t17173 + F::cast_from(0.5936111111111111111e-2_f64) * t17175 - F::cast_from(0.11872222222222222222e-1_f64) * t17180 + F::cast_from(0.35616666666666666666e-1_f64) * t17185 - F::cast_from(0.17808333333333333333e-1_f64) * t17189;
    t17488
}

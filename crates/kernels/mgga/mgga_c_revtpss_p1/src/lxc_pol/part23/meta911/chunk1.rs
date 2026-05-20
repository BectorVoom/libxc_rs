//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2927/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2927<F: Float>(t23475: F, t698: F, t41441: F, t63464: F, t77559: F, t77561: F, t77566: F, t77570: F, t77575: F, t77581: F, t77586: F, t77590: F, t77594: F) -> (F, F) {
    let t77858 = t698 * t23475;
    let t77860 = F::cast_from(0.20128333333333333333e0_f64) * t77559 - F::new(0.60385e0) * t77561 + F::cast_from(0.40256666666666666666e1_f64) * t77566 - F::cast_from(0.10064166666666666667e1_f64) * t77570 - F::cast_from(0.89459259259259259259e0_f64) * t77575 + F::cast_from(0.24528888888888888889e0_f64) * t41441 - F::cast_from(0.40256666666666666668e0_f64) * t63464 + F::new(0.60385e0) * t77581 - F::cast_from(0.20128333333333333333e0_f64) * t77586 - F::new(0.72462e1) * t77590 + F::new(0.36231e1) * t77594 + F::new(0.5519e-1) * t77858;
    (t77858, t77860)
}

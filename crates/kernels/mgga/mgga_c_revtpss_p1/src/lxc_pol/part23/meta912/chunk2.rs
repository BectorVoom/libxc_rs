//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2934/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2934<F: Float>(t41441: F, t63464: F, t77559: F, t77561: F, t77566: F, t77570: F, t77575: F, t77581: F, t77586: F, t77590: F, t77594: F, t77858: F) -> F {
    let t77974 = F::cast_from(0.34431666666666666667e0_f64) * t77559 - F::cast_from(0.103295e1_f64) * t77561 + F::cast_from(0.68863333333333333334e1_f64) * t77566 - F::cast_from(0.17215833333333333334e1_f64) * t77570 - F::cast_from(0.15302962962962962963e1_f64) * t77575 + F::cast_from(0.30872592592592592592e0_f64) * t41441 - F::cast_from(0.68863333333333333332e0_f64) * t63464 + F::cast_from(0.103295e1_f64) * t77581 - F::cast_from(0.34431666666666666667e0_f64) * t77586 - F::cast_from(0.123954e2_f64) * t77590 + F::cast_from(0.61977e1_f64) * t77594 + F::cast_from(0.69463333333333333333e-1_f64) * t77858;
    t77974
}

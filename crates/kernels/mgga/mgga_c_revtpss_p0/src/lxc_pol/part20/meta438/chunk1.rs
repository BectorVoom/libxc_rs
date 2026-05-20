//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1652/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1652<F: Float>(t43858: F, t43862: F, t43865: F, t43871: F, t43877: F, t43883: F, t43909: F, t43911: F, t43914: F, t43917: F, t43920: F, t43923: F, t43926: F, t43928: F) -> F {
    let t45134 = F::new(0.3529725e1) * t43909 - F::cast_from(0.23154444444444444445e0_f64) * t43911 - F::new(0.104195e0) * t43914 + F::new(0.62517e0) * t43917 - F::cast_from(0.13892666666666666667e0_f64) * t43920 - F::new(0.125034e1) * t43923 + F::new(0.83356e0) * t43926 + F::cast_from(0.27785333333333333333e0_f64) * t43928 - F::cast_from(0.76514814814814814814e0_f64) * t43858 - F::cast_from(0.15302962962962962963e1_f64) * t43862 - F::cast_from(0.91817777777777777776e0_f64) * t43865 - F::new(0.103295e1) * t43871 + F::new(0.309885e1) * t43877 + F::cast_from(0.27545333333333333332e1_f64) * t43883;
    t45134
}

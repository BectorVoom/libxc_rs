//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 911/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk911<F: Float>(t10030: F, t7244: F, t10024: F, t498: F, t515: F, t7230: F, t7231: F, t9843: F, t321: F, t3352: F, t1971: F, t2144: F, t333: F) -> (F, F, F, F, F) {
    let t45242 = t7244 * t10030;
    let t45244 = t7244 * t10024;
    let t45249 = t7230 * t7231 * t515 * t9843 * t498;
    let t45254 = t7230 * t3352 * t515 * t9843 * t321;
    let t45259 = t7230 * t1971 * t2144 * t9843 * t333;
    (t45242, t45244, t45249, t45254, t45259)
}

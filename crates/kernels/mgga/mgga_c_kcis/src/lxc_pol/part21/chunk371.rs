//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 371/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk371<F: Float>(t2155: F, t2157: F, t2144: F, t2148: F, t2151: F) -> (F, F) {
    let t2158 = t2155 * t2157;
    let t2161 = -F::cast_from(0.69505208333333333333e-3_f64) * t2158 + F::cast_from(0.69644166666666666665e-2_f64) * t2144;
    let t2165 = F::cast_from(0.1875e0_f64) * t2148 - F::cast_from(0.809375e-1_f64) * t2151;
    (t2161, t2165)
}

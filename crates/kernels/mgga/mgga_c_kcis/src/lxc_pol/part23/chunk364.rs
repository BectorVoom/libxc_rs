//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 364/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk364<F: Float>(t2155: F, t2157: F, t2144: F, t2148: F, t2151: F) -> (F, F) {
    let t2158 = t2155 * t2157;
    let t2161 = -F::new(0.69505208333333333333e-3) * t2158 + F::new(0.69644166666666666665e-2) * t2144;
    let t2165 = F::new(0.1875e0) * t2148 - F::new(0.809375e-1) * t2151;
    (t2161, t2165)
}

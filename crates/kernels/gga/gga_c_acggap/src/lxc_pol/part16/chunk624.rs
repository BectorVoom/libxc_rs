//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 624/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk624<F: Float>(t1089: F, t368: F, t5959: F, t1487: F, t495: F, t1734: F, t322: F, t175: F, t384: F, t1426: F, t5651: F, t1817: F, t3343: F) -> (F, F, F, F, F, F, F, F) {
    let t5961 = t1089 * t368 * t5959;
    let t5964 = t495 * t1487;
    let t5966 = t1089 * t368 * t5964;
    let t5969 = t1734 * t322;
    let t5971 = t1089 * t175 * t5969;
    let t5972 = t384 * t5971;
    let t5975 = t1426 * t175 * t5651;
    let t5978 = t3343 * t1817;
    (t5961, t5964, t5966, t5969, t5971, t5972, t5975, t5978)
}

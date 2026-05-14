//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 352/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk352<F: Float>(t2140: F, t334: F, t688: F, t125: F, t137: F, t86: F, t165: F, t113: F, t153: F, t160: F, t62: F) -> (F, F, F, F, F) {
    let t2141 = t688 * t334 * t2140;
    let t2144 = t86 * t125 * t137;
    let t2146 = -0.69505208333333333333e-3 * t2141 + 0.99491666666666666664e-2 * t2144;
    let t2147 = t2146 * t165;
    let t2148 = t153 * t113;
    let t2150 = t62 * t160;
    (t2144, t2146, t2147, t2148, t2150)
}

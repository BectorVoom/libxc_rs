//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 214/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk214<F: Float>(t128: F, t72: F, t686: F, t3: F, t66: F, t124: F) -> (F, F, F, F) {
    let t691 = f64::sqrt(t128);
    let t692 = t691 * t72;
    let t693 = t692 * t686;
    let t696 = F::new(1.0) / t66 / t3;
    let t697 = t124 * t696;
    (t692, t693, t696, t697)
}

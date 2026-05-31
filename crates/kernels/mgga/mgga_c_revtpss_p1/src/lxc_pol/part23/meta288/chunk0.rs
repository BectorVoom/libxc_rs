//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1516/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1516<F: Float>(t1071: F, t3057: F, t992: F, t338: F) -> (F, F, F, F) {
    let t11187 = t3057 * t1071;
    let t11198 = t992 * t992;
    let t11199 = F::cast_from(1.0_f64) / t11198;
    let t11200 = t338 * t11199;
    (t11187, t11198, t11199, t11200)
}

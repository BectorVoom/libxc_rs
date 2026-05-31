//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 1841/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1841<F: Float>(t1025: F, t11817: F, t271: F, t2857: F, t283: F) -> (F, F, F) {
    let t11818 = t1025 * t11817;
    let t11821 = F::cast_from(1.0_f64) / t271 / t2857;
    let t11852 = F::cast_from(1.0_f64) / t283 / t2857;
    (t11818, t11821, t11852)
}

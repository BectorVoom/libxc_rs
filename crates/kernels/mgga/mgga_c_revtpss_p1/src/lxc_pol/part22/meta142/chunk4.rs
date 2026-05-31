//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 955/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk955<F: Float>(t1132: F, t3399: F, t3356: F, t406: F) -> (F, F, F) {
    let t3400 = t1132 * t3399;
    let t3402 = F::cast_from(0.39862222222222222223e0_f64) * t3356;
    let t3407 = F::cast_from(1.0_f64)/F::sqrt(t406);
    (t3400, t3402, t3407)
}

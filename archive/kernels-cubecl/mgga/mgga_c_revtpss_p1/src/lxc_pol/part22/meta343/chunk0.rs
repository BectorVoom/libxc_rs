//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 1818/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1818<F: Float>(t11132: F, t11337: F, t2966: F, t944: F, t302: F) -> (F, F, F, F) {
    let t11422 = F::cast_from(0.16068111111111111111e1_f64) * t11132;
    let t11423 = F::cast_from(0.46308888888888888888e0_f64) * t11337;
    let t11449 = F::cast_from(1.0_f64) / t2966 / t944;
    let t11450 = t302 * t11449;
    (t11422, t11423, t11449, t11450)
}

//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 491/1093 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk491<F: Float>(t1063: F, t4817: F, t1670: F, t3172: F, t1041: F, t1065: F, t1651: F, t1062: F, t1659: F, t3204: F, t127: F, t1663: F, t371: F, t1025: F, t225: F, t4746: F) -> (F, F, F, F, F, F, F, F, F) {
    let t4818 = t1063 * t4817;
    let t4820 = t3172 * t1670;
    let t4821 = t1041 * t4820;
    let t4823 = t1065 * t1651;
    let t4834 = t1659 * t1062;
    let t4837 = t3204 * t1062;
    let t4845 = t371 * t127 * t1663;
    let t4846 = t1025 * t4845;
    let t4857 = t4746 * t225;
    (t4818, t4820, t4821, t4823, t4834, t4837, t4845, t4846, t4857)
}

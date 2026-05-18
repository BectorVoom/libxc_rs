//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 841/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk841<F: Float>(t4181: F, t4801: F, t1042: F, t2852: F, t3181: F, t1592: F, t3109: F, t247: F, t1063: F, t1670: F, t3172: F, t1041: F) -> (F, F, F, F, F, F, F, F, F) {
    let t4802 = t4801 * t4181;
    let t4803 = t1042 * t4802;
    let t4806 = t3181 * t2852;
    let t4807 = t4806 * t4181;
    let t4808 = t1042 * t4807;
    let t4816 = t3109 * t1592;
    let t4817 = t247 * t4816;
    let t4818 = t1063 * t4817;
    let t4820 = t3172 * t1670;
    let t4821 = t1041 * t4820;
    (t4802, t4803, t4806, t4807, t4808, t4817, t4818, t4820, t4821)
}

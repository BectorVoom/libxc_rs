//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 554/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk554<F: Float>(t247: F, t4816: F, t1063: F, t1670: F, t3172: F, t1041: F, t1065: F, t1651: F, t906: F, t1042: F, t1066: F, t4583: F) -> (F, F, F, F) {
    let t4817 = t247 * t4816;
    let t4818 = t1063 * t4817;
    let t4820 = t3172 * t1670;
    let t4821 = t1041 * t4820;
    let t4823 = t1065 * t1651;
    let t4824 = t4823 * t906;
    let t4825 = t1042 * t4824;
    let t4830 = t1066 * t4583;
    (t4818, t4821, t4825, t4830)
}

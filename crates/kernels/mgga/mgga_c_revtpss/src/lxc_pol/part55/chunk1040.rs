//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1040/1151 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1040<F: Float>(t121204: F, t1868: F, t9818: F, t121232: F, t1353: F, t1903: F, t120956: F, t1414: F, t828: F, t120967: F, t125627: F, t247: F, t3938: F, t120975: F, t1885: F, t121034: F, t1390: F, t32192: F, t5727: F, t8583: F) -> (F, F, F, F, F, F, F) {
    let t125662 = t9818 * t121204 * t1868;
    let t125663 = t121232 * t125662;
    let t125668 = t1903 * t1353;
    let t125671 = t120956 * t1414 * t828 * t125668;
    let t125677 = t120967 * t247 * t125627 * t3938;
    let t125706 = t120975 * t1885;
    let t125717 = t8583 * t121034 * t32192 * t1390 * t828 * t5727;
    (t125662, t125663, t125668, t125671, t125677, t125706, t125717)
}

//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1317/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1317<F: Float>(t112387: F, t116123: F, t116416: F, t116513: F, t116958: F, t116960: F, t116965: F, t116971: F, t116973: F, t116976: F, t116979: F, t116983: F, t33035: F, t33061: F, t9652: F, t9667: F, t9672: F, t9922: F) -> (F,) {
    let t116988 = 0.18518518518518518519e-1 * t116416 * t9667 + 0.18518518518518518519e-1 * t116123 * t9667 + 0.99491666666666666664e-2 * t116958 + 0.69444444444444444446e-2 * t116960 * t33061 + 0.69444444444444444446e-2 * t116960 * t33035 + 0.26805555555555555556e-2 * t116965 * t33035 + 0.69444444444444444446e-2 * t116513 * t33061 - t116971 + 0.16581944444444444444e-2 * t116973 - 0.24872916666666666666e-2 * t116976 + t116979 + 0.10416666666666666667e-1 * t112387 * t9922 + 0.20833333333333333334e-1 * t116983 * t9672 + 0.20833333333333333334e-1 * t116983 * t9652;
    (t116988,)
}

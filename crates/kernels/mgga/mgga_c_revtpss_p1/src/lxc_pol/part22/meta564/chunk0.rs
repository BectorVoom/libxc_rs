//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2401/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2401<F: Float>(t1828: F, t3568: F, t1277: F, t1294: F, t5497: F, t3737: F, t17288: F, t487: F) -> (F, F, F, F, F) {
    let t18102 = t1828 * t3568;
    let t18103 = t1277 * t18102;
    let t18108 = t5497 * t1294;
    let t18109 = t3737 * t18108;
    let t18114 = t17288 * t487;
    (t18102, t18103, t18108, t18109, t18114)
}

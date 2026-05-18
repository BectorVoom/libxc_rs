//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1179/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1179<F: Float>(t121174: F, t125662: F, t124: F, t1380: F, t1903: F, t800: F, t32705: F, t32710: F, t5710: F, t8477: F, t32272: F, t33970: F) -> (F, F, F, F, F) {
    let t125826 = t121174 * t125662;
    let t125830 = t1380 * t800 * t124 * t1903;
    let t125831 = t32705 * t125830;
    let t125833 = t32710 * t125830;
    let t125849 = t8477 * t5710;
    let t125855 = t32272 * t33970;
    (t125826, t125831, t125833, t125849, t125855)
}

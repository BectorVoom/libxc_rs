//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 970/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk970<F: Float>(t121126: F, t32206: F, t5673: F, t5727: F, t25924: F, t121174: F, t125662: F, t124: F, t1380: F, t1903: F, t800: F, t32705: F, t32710: F, t5659: F, t7301: F, t5710: F, t8477: F) -> (F, F, F, F, F, F, F) {
    let t125819 = t32206 * t5673 * t121126 * t5727;
    let t125821 = t25924 * t5727;
    let t125826 = t121174 * t125662;
    let t125830 = t1380 * t800 * t124 * t1903;
    let t125831 = t32705 * t125830;
    let t125833 = t32710 * t125830;
    let t125835 = t7301 * t5659;
    let t125849 = t8477 * t5710;
    (t125819, t125821, t125826, t125831, t125833, t125835, t125849)
}

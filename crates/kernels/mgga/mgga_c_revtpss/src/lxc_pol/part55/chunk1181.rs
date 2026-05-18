//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1181/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1181<F: Float>(t121175: F, t13847: F, t1903: F, t121232: F, t121174: F, t120980: F, t1873: F, t32265: F, t32269: F, t125849: F, t552: F, t8590: F) -> (F, F, F, F, F) {
    let t125900 = t13847 * t121175 * t1903;
    let t125901 = t121232 * t125900;
    let t125903 = t121174 * t125900;
    let t125922 = t120980 * t1873;
    let t125923 = t32265 * t125922;
    let t125925 = t32269 * t125922;
    let t125928 = t125849 * t8590 * t552;
    (t125901, t125903, t125923, t125925, t125928)
}

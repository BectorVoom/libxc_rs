//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1124/1200 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1124<F: Float>(t121175: F, t13847: F, t1903: F, t121232: F, t121174: F, t25876: F, t545: F, t5774: F, t27864: F, t8707: F, t14224: F, t7301: F) -> (F, F, F, F, F) {
    let t125900 = t13847 * t121175 * t1903;
    let t125901 = t121232 * t125900;
    let t125903 = t121174 * t125900;
    let t125906 = t25876 * t545 * t5774;
    let t125915 = t8707 * t27864;
    let t125918 = t7301 * t14224;
    (t125901, t125903, t125906, t125915, t125918)
}

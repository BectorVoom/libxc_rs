//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 809/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk809<F: Float>(t1086: F, t1678: F, t994: F, t1647: F, t3316: F, t15669: F, t378: F, t1716: F, t2435: F) -> (F, F, F, F) {
    let t16543 = t1086 * t1678;
    let t16544 = t994 * t16543;
    let t16584 = t1647 * t3316;
    let t16600 = t15669 * t378;
    let t16706 = t2435 * t1716;
    (t16544, t16584, t16600, t16706)
}

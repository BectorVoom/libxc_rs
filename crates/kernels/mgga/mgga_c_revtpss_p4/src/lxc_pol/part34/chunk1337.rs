//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1337/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1337<F: Float>(t114222: F, t114431: F, t114766: F, t114824: F, t6941: F, t7953: F, t572: F, t5883: F, t7741: F, t22633: F, t7330: F, t105823: F, t5920: F) -> (F, F, F, F, F) {
    let t114826 = t114222 + t114431 + t114766 + t114824;
    let t114838 = F::cast_from(9.0_f64) * t6941 * t7953;
    let t114841 = F::cast_from(18.0_f64) * t572 * t5883 * t7741;
    let t114844 = F::cast_from(6.0_f64) * t572 * t7330 * t22633;
    let t114847 = F::cast_from(18.0_f64) * t572 * t105823 * t5920;
    (t114826, t114838, t114841, t114844, t114847)
}

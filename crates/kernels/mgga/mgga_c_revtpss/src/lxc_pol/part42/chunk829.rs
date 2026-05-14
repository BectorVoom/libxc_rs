//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 829/1363 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk829<F: Float>(t108: F, t5911: F, t105: F, t109: F, t1507: F, t1510: F, t5896: F, t5899: F, t5902: F, t5908: F, t97: F) -> (F,) {
    let t5912 = t108 * t5911;
    let t5915 = 10.0 / 9.0 * t97 * t5896 + 5.0 / 3.0 * t97 * t5899 + 40.0 / 9.0 * t5902 * t109 - 50.0 / 9.0 * t1507 * t1510 + 10.0 / 9.0 * t105 * t5908 + 5.0 / 3.0 * t105 * t5912;
    (t5915,)
}

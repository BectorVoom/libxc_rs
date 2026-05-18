//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1346/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1346<F: Float>(t114800: F, t25082: F, t8717: F, t1450: F, t2014: F, t2033: F, t22813: F, t22633: F, t94: F, t1937: F, t29508: F, t7735: F) -> (F, F, F, F) {
    let t114803 = F::new(9.0) * t25082 * t8717 * t114800;
    let t114807 = F::new(6.0) * t2014 * t22813 * t2033 * t1450;
    let t114812 = t94 * t22633;
    let t114814 = F::new(2.0) * t114812 * t1937;
    let t114816 = F::new(6.0) * t29508 * t7735;
    (t114803, t114807, t114814, t114816)
}

//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2192/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2192<F: Float>(t108710: F, t1937: F, t21881: F, t94: F, t29508: F, t6993: F, t25082: F, t86815: F, t8717: F, t7003: F, t27123: F, t7735: F) -> (F, F, F, F, F, F) {
    let t108712 = F::new(2.0) * t108710 * t1937;
    let t108714 = t94 * t21881;
    let t108716 = F::new(2.0) * t108714 * t1937;
    let t108718 = F::new(2.0) * t29508 * t6993;
    let t108721 = F::new(6.0) * t25082 * t8717 * t86815;
    let t108723 = F::new(2.0) * t29508 * t7003;
    let t108725 = F::new(4.0) * t27123 * t7735;
    (t108712, t108716, t108718, t108721, t108723, t108725)
}

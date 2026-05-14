//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1134/1170 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1134<F: Float>(t28196: F, t28197: F, t49654: F, t1450: F, t9628: F, t2014: F, t7237: F, t25082: F, t49560: F, t3813: F, t651: F, t7002: F, t18163: F, t7003: F, t25861: F, t4254: F) -> (F, F, F, F, F, F) {
    let t95001 = 6.0 * t28196 * t28197 * t49654;
    let t95002 = t1450 * t9628;
    let t95005 = 3.0 * t2014 * t7237 * t95002;
    let t95008 = 18.0 * t25082 * t28197 * t49560;
    let t95011 = 6.0 * t651 * t3813 * t7002;
    let t95013 = 6.0 * t18163 * t7003;
    let t95015 = 12.0 * t4254 * t25861;
    (t95001, t95005, t95008, t95011, t95013, t95015)
}

//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2072/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2072<F: Float>(t28177: F, t7235: F, t28056: F, t4254: F, t5517: F, t651: F, t7002: F, t2028: F, t27980: F, t13790: F, t4102: F, t685: F, t72: F) -> (F, F, F, F, F) {
    let t97661 = F::new(6.0) * t7235 * t28177;
    let t97663 = F::new(4.0) * t4254 * t28056;
    let t97666 = F::new(4.0) * t651 * t5517 * t7002;
    let t97676 = t2028 * t27980;
    let t97680 = t13790 * t72 * t685 * t4102;
    (t97661, t97663, t97666, t97676, t97680)
}

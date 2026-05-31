//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2084/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2084<F: Float>(t5517: F, t651: F, t7002: F, t2028: F, t27980: F, t13790: F, t4102: F, t685: F, t72: F, t25875: F, t1444: F, t5740: F, t675: F) -> (F, F, F, F, F) {
    let t97666 = F::cast_from(4.0_f64) * t651 * t5517 * t7002;
    let t97676 = t2028 * t27980;
    let t97680 = t13790 * t72 * t685 * t4102;
    let t97682 = F::cast_from(0.51405703062096148812e-1_f64) * t25875 * t97676 * t97680;
    let t97685 = t5740 * t685 * t675 * t1444;
    (t97666, t97676, t97680, t97682, t97685)
}

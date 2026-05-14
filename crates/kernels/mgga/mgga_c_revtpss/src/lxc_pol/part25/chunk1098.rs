//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1098/1212 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1098<F: Float>(t5: F, t92618: F, t92649: F, t92682: F, t92715: F, t117: F, t25856: F, t4254: F, t13207: F, t1936: F, t651: F, t2322: F, t25851: F, t1310: F, t25832: F, t116: F, t25168: F) -> (F, F, F, F, F, F, F) {
    let t7 = piecewise3(0.0 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0;
    let t92718 = piecewise3(t8, 0.0, t92618 + t92649 + t92682 + t92715);
    let t92719 = t92718 * t117;
    let t92724 = 6.0 * t4254 * t25856;
    let t92727 = 2.0 * t651 * t13207 * t1936;
    let t92731 = 6.0 * t2322 * t25851;
    let t92733 = 6.0 * t4254 * t25851;
    let t92736 = 6.0 * t651 * t1310 * t25832;
    let t92737 = t25168 * t116;
    (t92719, t92724, t92727, t92731, t92733, t92736, t92737)
}

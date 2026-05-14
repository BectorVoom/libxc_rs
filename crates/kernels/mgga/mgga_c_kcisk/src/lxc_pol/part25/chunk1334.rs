//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1334/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1334<F: Float>(t17951: F, t33097: F, t18163: F, t736: F, t17982: F, t654: F, t33098: F, t11659: F, t2580: F, t17825: F, t1873: F, t17924: F, t9704: F, t112046: F, t9972: F, t33109: F, t34316: F) -> (F, F, F, F, F, F, F, F) {
    let t117306 = t33097 * t17951;
    let t117308 = t18163 * t736;
    let t117310 = t17982 * t654;
    let t117311 = t117310 * t33098;
    let t117313 = t11659 * t2580;
    let t117315 = t1873 * t17825;
    let t117317 = t9704 * t17924;
    let t117319 = t112046 * t9972;
    let t117321 = t34316 * t33109;
    (t117306, t117308, t117311, t117313, t117315, t117317, t117319, t117321)
}

//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2173/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2173<F: Float>(t3172: F, t4874: F, t3127: F, t4802: F, t1063: F, t4807: F, t11723: F, t11728: F, t11730: F, t11732: F, t11737: F, t11745: F, t15758: F, t3106: F, t4803: F, t4808: F, t4896: F) -> (F, F, F, F, F, F, F) {
    let t15769 = t3172 * t4874;
    let t15771 = F::cast_from(0.19055119163586549765e-3_f64) * t3127 * t15769;
    let t15772 = t3172 * t4802;
    let t15774 = F::cast_from(0.3811023832717309953e-3_f64) * t1063 * t15772;
    let t15775 = t3172 * t4807;
    let t15776 = t1063 * t15775;
    let t15779 = F::cast_from(0.85748036236139473944e-3_f64) * t15758 * t4896 + F::cast_from(0.95275595817932748827e-4_f64) * t11723 + F::cast_from(0.15879265969655458138e-3_f64) * t11728 + F::new(11.0) / F::new(324.0) * t11730 + t11732 / F::new(81.0) + t11737 + F::cast_from(0.30488190661738479624e-2_f64) * t3106 * t4803 - F::cast_from(0.2540682555144873302e-2_f64) * t3106 * t4808 - t15771 - t15774 + F::cast_from(0.31758531939310916276e-3_f64) * t15776 - F::cast_from(0.19055119163586549765e-3_f64) * t11745;
    (t15769, t15771, t15772, t15774, t15775, t15776, t15779)
}

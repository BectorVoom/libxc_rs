//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3738/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3738<F: Float>(t17448: F, t17451: F, t1121: F, t6587: F, t13148: F, t70916: F, t13142: F, t12772: F, t21218: F, t3625: F, t12784: F, t12787: F, t12855: F, t12910: F, t17429: F, t17459: F, t17713: F, t17729: F, t17730: F, t17736: F, t17750: F, t20297: F, t20838: F, t21008: F, t21119: F, t21164: F, t21257: F, t3626: F, t3720: F, t5354: F, t5407: F, t57040: F, t57571: F, t58791: F) -> F {
    let t71020 = t17448 * t17451;
    let t71029 = t6587 * t1121;
    let t71036 = t13148 * t70916;
    let t71039 = t13142 * t70916;
    let t71047 = t3625 * t12772 * t21218;
    let t71053 = F::cast_from(0.17149607247227894789e-2_f64) * t12910 * t3720 * t21164 * t17459 - F::cast_from(0.3811023832717309953e-3_f64) * t71020 + F::cast_from(0.57165357490759649296e-3_f64) * t58791 - F::cast_from(0.17149607247227894789e-2_f64) * t12855 * t3720 * t21257 * t21119 - F::cast_from(0.85748036236139473944e-3_f64) * t57040 * t5354 - F::cast_from(0.57165357490759649296e-3_f64) * t17736 * t3626 * t71029 * t17730 - F::cast_from(0.85748036236139473944e-3_f64) * t17429 * t20838 - F::cast_from(0.13719685797782315831e-1_f64) * t71036 * t17713 + F::cast_from(0.13719685797782315831e-1_f64) * t71039 * t17750 + F::cast_from(0.47637797908966374414e-3_f64) * t12784 * t21008 + F::cast_from(0.30488190661738479624e-2_f64) * t57571 * t5407 - F::cast_from(0.19055119163586549765e-3_f64) * t71047 - F::cast_from(0.28582678745379824648e-2_f64) * t17729 * t12787 * t20297 * t17730;
    t71053
}

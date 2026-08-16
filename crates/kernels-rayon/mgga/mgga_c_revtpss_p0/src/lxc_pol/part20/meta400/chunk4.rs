//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1487/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1487(t11773: f64, t11865: f64, t11941: f64, t11942: f64, t127: f64, t371: f64, t11937: f64, t11947: f64, t3205: f64, t3206: f64, t676: f64, t1063: f64, t1066: f64, t11286: f64, t11663: f64, t11687: f64, t11774: f64, t11776: f64, t11859: f64, t11994: f64, t12024: f64, t15609: f64, t15758: f64, t225: f64, t247: f64, t3096: f64, t3117: f64, t366: f64, t372: f64, t375: f64, t41310: f64, t42033: f64, t42149: f64) -> f64 {
    let t42155 = t11865 * t11773;
    let t42170 = t11941 * t371 * t127 * t11942;
    let t42172 = t11947 * t11937;
    let t42176 = t3205 * t371 * t676 * t3206;
    let t42184 = 0.57165357490759649296e-3_f64 * t42149 + 0.21437009059034868486e-3_f64 * t42033 * t225 * t366 * t375 - 0.34299214494455789577e-2_f64 * t42155 * t11776 - 0.17149607247227894789e-2_f64 * t11774 * t372 * t12024 * t3096 - 0.34299214494455789578e-2_f64 * t1063 * t247 * t1066 * t41310 - 0.28582678745379824648e-2_f64 * t11994 * t11286 - 0.34299214494455789578e-2_f64 * t42170 - 0.18292914397043087774e-1_f64 * t42172 - 0.57165357490759649296e-3_f64 * t42176 + 0.34299214494455789578e-2_f64 * t15758 * t11663 - 0.51448821741683684368e-2_f64 * t11859 * t3117 * t11687 * t15609;
    t42184
}

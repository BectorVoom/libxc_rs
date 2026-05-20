//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1487/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1487<F: Float>(t11773: F, t11865: F, t11941: F, t11942: F, t127: F, t371: F, t11937: F, t11947: F, t3205: F, t3206: F, t676: F, t1063: F, t1066: F, t11286: F, t11663: F, t11687: F, t11774: F, t11776: F, t11859: F, t11994: F, t12024: F, t15609: F, t15758: F, t225: F, t247: F, t3096: F, t3117: F, t366: F, t372: F, t375: F, t41310: F, t42033: F, t42149: F) -> F {
    let t42155 = t11865 * t11773;
    let t42170 = t11941 * t371 * t127 * t11942;
    let t42172 = t11947 * t11937;
    let t42176 = t3205 * t371 * t676 * t3206;
    let t42184 = F::cast_from(0.57165357490759649296e-3_f64) * t42149 + F::cast_from(0.21437009059034868486e-3_f64) * t42033 * t225 * t366 * t375 - F::cast_from(0.34299214494455789577e-2_f64) * t42155 * t11776 - F::cast_from(0.17149607247227894789e-2_f64) * t11774 * t372 * t12024 * t3096 - F::cast_from(0.34299214494455789578e-2_f64) * t1063 * t247 * t1066 * t41310 - F::cast_from(0.28582678745379824648e-2_f64) * t11994 * t11286 - F::cast_from(0.34299214494455789578e-2_f64) * t42170 - F::cast_from(0.18292914397043087774e-1_f64) * t42172 - F::cast_from(0.57165357490759649296e-3_f64) * t42176 + F::cast_from(0.34299214494455789578e-2_f64) * t15758 * t11663 - F::cast_from(0.51448821741683684368e-2_f64) * t11859 * t3117 * t11687 * t15609;
    t42184
}

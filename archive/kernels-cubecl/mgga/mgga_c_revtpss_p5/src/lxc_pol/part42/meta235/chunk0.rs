//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 903/1505 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk903<F: Float>(t247: F, t6326: F, t1066: F, t6096: F, t373: F, t6244: F, t371: F, t372: F, t1041: F, t1063: F, t1671: F, t1675: F, t3150: F, t3161: F, t3203: F, t3205: F, t375: F, t4834: F, t4846: F, t4879: F, t4925: F, t6302: F, t6308: F, t6312: F, t6318: F, t6323: F) -> (F, F, F, F, F) {
    let t6327 = t247 * t6326;
    let t6330 = t1066 * t6096;
    let t6331 = t247 * t6330;
    let t6337 = t373 * t6244;
    let t6339 = t371 * t372 * t6337;
    let t6342 = F::cast_from(0.21437009059034868486e-3_f64) * t1041 * t6302 + F::cast_from(0.42874018118069736972e-3_f64) * t3150 * t6308 - F::cast_from(0.21437009059034868486e-3_f64) * t3161 * t6312 + F::cast_from(0.42874018118069736972e-3_f64) * t4879 * t1671 + F::cast_from(0.21437009059034868486e-3_f64) * t6318 * t375 - F::cast_from(0.28582678745379824648e-3_f64) * t4846 + F::cast_from(0.14291339372689912324e-3_f64) * t1063 * t6323 + F::cast_from(0.23818898954483187207e-3_f64) * t1063 * t6327 - F::cast_from(0.28582678745379824648e-3_f64) * t1063 * t6331 - t3203 + t4925 / F::cast_from(432.0_f64) + F::cast_from(0.28582678745379824648e-3_f64) * t4834 * t1675 + F::cast_from(0.42874018118069736972e-3_f64) * t3205 * t6339;
    (t6327, t6331, t6337, t6339, t6342)
}

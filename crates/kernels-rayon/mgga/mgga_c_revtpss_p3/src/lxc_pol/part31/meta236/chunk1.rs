//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1059/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1059(t1041: f64, t1063: f64, t1671: f64, t1675: f64, t3150: f64, t3161: f64, t3203: f64, t3205: f64, t375: f64, t4834: f64, t4846: f64, t4879: f64, t4925: f64, t6302: f64, t6308: f64, t6312: f64, t6318: f64, t6323: f64, t6327: f64, t6331: f64, t6339: f64) -> f64 {
    let t6342 = 0.21437009059034868486e-3_f64 * t1041 * t6302 + 0.42874018118069736972e-3_f64 * t3150 * t6308 - 0.21437009059034868486e-3_f64 * t3161 * t6312 + 0.42874018118069736972e-3_f64 * t4879 * t1671 + 0.21437009059034868486e-3_f64 * t6318 * t375 - 0.28582678745379824648e-3_f64 * t4846 + 0.14291339372689912324e-3_f64 * t1063 * t6323 + 0.23818898954483187207e-3_f64 * t1063 * t6327 - 0.28582678745379824648e-3_f64 * t1063 * t6331 - t3203 + t4925 / 432.0_f64 + 0.28582678745379824648e-3_f64 * t4834 * t1675 + 0.42874018118069736972e-3_f64 * t3205 * t6339;
    t6342
}

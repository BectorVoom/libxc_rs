//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1257/1422 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk1257(t11947: f64, t15745: f64, t16134: f64, t16160: f64, t16190: f64, t1665: f64, t1671: f64, t20017: f64, t20021: f64, t20025: f64, t20030: f64, t20034: f64, t3188: f64, t6327: f64, t6339: f64) -> f64 {
    let t20036 = -0.22866142996303859718e-2_f64 * t11947 * t6339 + 0.28582678745379824648e-3_f64 * t20017 - 0.14291339372689912324e-3_f64 * t20021 + 0.22866142996303859718e-2_f64 * t15745 * t1665 - 0.28582678745379824648e-3_f64 * t20025 + t16134 + 0.23818898954483187207e-3_f64 * t3188 * t6327 + 0.28582678745379824648e-3_f64 * t20030 - 0.22866142996303859718e-2_f64 * t16190 * t1671 + 0.28582678745379824648e-3_f64 * t20034 + t16160;
    t20036
}

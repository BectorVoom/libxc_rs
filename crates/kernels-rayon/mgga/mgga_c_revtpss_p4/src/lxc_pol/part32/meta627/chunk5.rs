//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 2007/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2007(t105936: f64, t95822: f64, t102930: f64, t102934: f64, t102937: f64, t102939: f64, t102941: f64, t102943: f64, t102945: f64, t1579: f64, t18784: f64, t2061: f64, t25383: f64, t28340: f64, t29698: f64, t30342: f64, t4533: f64, t6071: f64, t7070: f64, t7071: f64, t7398: f64, t7424: f64, t7997: f64) -> f64 {
    let t110236 = t95822 * t105936;
    let t110242 = 0.17347256376410398924e1_f64 * t7070 * t7071 * t7997 * t4533 - 0.4336814094102599731e0_f64 * t29698 * t7424 + 0.17347256376410398924e1_f64 * t7070 * t7071 * t28340 * t1579 + 0.8673628188205199462e0_f64 * t7070 * t7071 * t2061 * t18784 - t102930 + 0.17347256376410398924e1_f64 * t25383 * t30342 + t102934 - t102937 + t102939 - t102941 + 0.28912093960683998207e-1_f64 * t110236 + t102943 - t102945 + 0.8673628188205199462e0_f64 * t7070 * t7071 * t7398 * t6071;
    t110242
}

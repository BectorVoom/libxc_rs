//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 2043/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2043(t670: f64, t7968: f64, t102019: f64, t109150: f64, t109368: f64, t110054: f64, t110110: f64, t1312: f64, t1518: f64, t18245: f64, t2055: f64, t21881: f64, t26399: f64, t28653: f64, t28658: f64, t34251: f64, t4292: f64, t5920: f64, t7359: f64, t7373: f64, t75439: f64, t85360: f64) -> (f64, f64) {
    let t111018 = t7968 * t670;
    let t111039 = 4.0_f64 * t102019 * t1518 + 4.0_f64 * t109150 * t2055 + 2.0_f64 * t109368 * t1312 + 2.0_f64 * t110110 * t670 + 4.0_f64 * t111018 * t1518 + 2.0_f64 * t18245 * t7373 + 2.0_f64 * t2055 * t75439 + 2.0_f64 * t2055 * t85360 + 2.0_f64 * t21881 * t7359 + 2.0_f64 * t26399 * t5920 + 4.0_f64 * t28653 * t4292 + 2.0_f64 * t28658 * t5920 + 4.0_f64 * t34251 * t4292 + t110054;
    (t111018, t111039)
}

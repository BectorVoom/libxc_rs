//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1098/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1098(t10416: f64, t1312: f64, t13435: f64, t13440: f64, t2055: f64, t2322: f64, t2371: f64, t26153: f64, t26210: f64, t26399: f64, t26676: f64, t5523: f64, t670: f64, t7359: f64, t7373: f64) -> f64 {
    let t26699 = 2.0_f64 * t10416 * t2055 + 2.0_f64 * t1312 * t26153 + 4.0_f64 * t13435 * t2055 + 2.0_f64 * t13440 * t2055 + 4.0_f64 * t2322 * t7373 + 2.0_f64 * t2371 * t7359 + 4.0_f64 * t26399 * t670 + 4.0_f64 * t5523 * t7373 + t26210 + 2.0_f64 * t26676;
    t26699
}

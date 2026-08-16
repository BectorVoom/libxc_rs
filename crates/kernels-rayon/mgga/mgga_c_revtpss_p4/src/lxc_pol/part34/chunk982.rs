//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 982/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk982(t1583: f64, t6079: f64, t10592: f64, t10596: f64, t10604: f64, t10611: f64, t11064: f64, t198: f64, t207: f64, t23191: f64, t23193: f64, t23213: f64, t23215: f64, t23218: f64, t23220: f64, t23223: f64, t9524: f64, t9542: f64) -> (f64, f64) {
    let t23429 = t6079 * t1583;
    let t23434 = 2.0_f64 * t11064 * t198 * t207 * t23429 + t10592 - t10596 - t10604 - t10611 + t23191 + t23193 + t23213 + t23215 + t23218 + t23220 + t23223 - t9524 + t9542;
    (t23429, t23434)
}

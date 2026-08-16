//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1317/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1317(t10509: f64, t123: f64, t2465: f64, t213: f64, t2760: f64, t215: f64, t231: f64, t268: f64, t836: f64, t2798: f64, t2722: f64, t675: f64) -> (f64, f64, f64, f64, f64) {
    let t10510 = t123 * t10509;
    let t10511 = t2465 * t10510;
    let t10513 = t213 * t2760;
    let t10518 = t268 * t215 * t836 * t231;
    let t10519 = t2798 * t10518;
    let t10521 = t675 * t2722;
    (t10510, t10511, t10513, t10519, t10521)
}

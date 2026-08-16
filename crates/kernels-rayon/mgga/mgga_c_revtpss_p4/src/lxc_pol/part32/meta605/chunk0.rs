//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1943/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1943(t105944: f64, t1955: f64, t5978: f64, t886: f64, t1558: f64, t231: f64, t4533: f64, t6048: f64, t836: f64, t6071: f64, t105945: f64, t7063: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t106275 = t1955 * t105944;
    let t106290 = t5978 * t886;
    let t106302 = t4533 * t1558 * t231;
    let t106360 = t6048 * t836 * t231;
    let t106365 = t6071 * t836 * t231;
    let t106387 = t7063 * t105945;
    (t106275, t106290, t106302, t106360, t106365, t106387)
}

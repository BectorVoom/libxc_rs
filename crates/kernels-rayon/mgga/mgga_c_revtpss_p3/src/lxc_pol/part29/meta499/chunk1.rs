//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1815/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1815(t531: f64, t8107: f64, t7238: f64, t2014: f64, t2056: f64, t2093: f64, t2108: f64, t27123: f64, t27126: f64, t27833: f64, t28167: f64, t28760: f64, t28927: f64, t28929: f64, t28932: f64, t28935: f64, t4248: f64, t5787: f64, t651: f64, t7235: f64, t7367: f64, t7374: f64, t7489: f64, t7732: f64, t7898: f64, t8079: f64, t8109: f64) -> (f64, f64, f64) {
    let t28938 = t531 * t8107;
    let t28939 = t28938 * t7238;
    let t28942 = t2014 * t28927 + 3.0_f64 * t2014 * t28932 + 3.0_f64 * t2014 * t28935 + 3.0_f64 * t2014 * t28939 - 2.0_f64 * t2056 * t27123 - 2.0_f64 * t2056 * t27126 + t2093 * t5787 + t2108 * t27833 + 6.0_f64 * t28167 * t28929 - 2.0_f64 * t28760 * t651 - 2.0_f64 * t4248 * t7374 + 3.0_f64 * t7235 * t8079 + t7235 * t8109 - 2.0_f64 * t7367 * t7732 + 3.0_f64 * t7489 * t7898;
    (t28938, t28939, t28942)
}

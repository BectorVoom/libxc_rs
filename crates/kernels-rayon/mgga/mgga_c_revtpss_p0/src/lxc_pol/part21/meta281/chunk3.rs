//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1515/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1515(t10178: f64, t9689: f64, t3889: f64, t566: f64, t1343: f64, t1353: f64, t1450: f64, t198: f64, t4139: f64, t4140: f64, t532: f64, t5536: f64, t9524: f64, t9542: f64, t9590: f64, t9593: f64, t9598: f64, t9599: f64, t9628: f64, t9854: f64, t9857: f64, t9859: f64, t9862: f64, t9865: f64, t9868: f64) -> (f64, f64) {
    let t10179 = t9689 + t10178;
    let t10186 = t566 * t3889;
    let t10190 = t10179 * t1450 * t198 * t532 + 2.0_f64 * t198 * t532 * t9590 * t9593 + 18.0_f64 * t10186 * t1353 * t5536 + 3.0_f64 * t1343 * t198 * t9628 - 9.0_f64 * t1353 * t4139 * t9599 + 9.0_f64 * t3889 * t4139 * t4140 - t9524 + t9542 + t9598 + t9854 - t9857 - t9859 + t9862 + t9865 + t9868;
    (t10179, t10190)
}

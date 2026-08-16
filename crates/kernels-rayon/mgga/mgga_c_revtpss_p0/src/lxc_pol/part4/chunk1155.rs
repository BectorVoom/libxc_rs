//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1155/1428 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk1155(t13750: f64, t14088: f64, t14279: f64, t14302: f64, t1343: f64, t13664: f64, t13667: f64, t13669: f64, t13671: f64, t13673: f64, t13674: f64, t13682: f64, t13683: f64, t13716: f64, t13885: f64, t13886: f64, t13888: f64, t1450: f64, t198: f64, t3889: f64, t4135: f64, t4139: f64, t4144: f64, t532: f64, t5532: f64, t5541: f64, t5542: f64, t9524: f64, t9542: f64, t9854: f64, t9865: f64, t9868: f64) -> f64 {
    let t14304 = t13750 + t14088 + t14279 + t14302;
    let t14308 = t14304 * t1450 * t198 * t532 + 3.0_f64 * t1343 * t13716 * t198 + 2.0_f64 * t13674 * t4144 * t5541 + 3.0_f64 * t3889 * t4139 * t5532 - t4135 * t5541 * t5542 - t13664 + t13667 + t13669 - t13671 + t13673 + t13682 + t13683 - t13885 + t13886 + t13888 - t9524 + t9542 - t9854 + t9865 + t9868;
    t14308
}

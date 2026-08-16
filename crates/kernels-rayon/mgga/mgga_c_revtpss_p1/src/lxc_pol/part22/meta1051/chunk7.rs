//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3712/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3712(t482: f64, t69623: f64, t1042: f64, t1261: f64, t17202: f64, t17448: f64, t17558: f64, t17569: f64, t17669: f64, t17796: f64, t3610: f64, t3611: f64, t44170: f64, t44343: f64, t44698: f64, t5381: f64, t5407: f64, t56254: f64, t57098: f64, t57100: f64, t57114: f64, t58983: f64, t65829: f64, t65947: f64, t6631: f64, t6635: f64) -> (f64, f64) {
    let t70343 = t482 * t69623;
    let t70361 = -0.85748036236139473944e-3_f64 * t1261 * t1042 * t17202 * t65829 - 0.34299214494455789578e-2_f64 * t1261 * t1042 * t56254 * t65947 + 0.47637797908966374413e-3_f64 * t5381 * t17558 + 0.42874018118069736972e-3_f64 * t44170 * t6631 - 0.21437009059034868486e-3_f64 * t44698 * t6635 - 0.42874018118069736972e-3_f64 * t3610 * t1042 * t70343 * t3611 - 0.47637797908966374413e-3_f64 * t17569 * t17796 + 0.23289590088828005269e-2_f64 * t1261 * t1042 * t58983 * t65947 + 0.3811023832717309953e-3_f64 * t57098 - 0.57165357490759649296e-3_f64 * t57100 * t5407 - 0.57165357490759649296e-3_f64 * t17448 * t17669 - 0.3811023832717309953e-3_f64 * t57114 + t44343 / 648.0_f64;
    (t70343, t70361)
}

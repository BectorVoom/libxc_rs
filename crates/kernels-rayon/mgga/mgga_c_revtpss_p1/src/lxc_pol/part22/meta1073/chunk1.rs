//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3849/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3849(t22263: f64, t9775: f64, t1412: f64, t6861: f64, t2661: f64, t3938: f64, t3992: f64, t5608: f64, t5659: f64, t1399: f64, t22025: f64, t1353: f64, t13902: f64, t13907: f64, t1392: f64, t1394: f64, t21969: f64, t22229: f64, t22237: f64, t22240: f64, t22245: f64, t22246: f64, t22249: f64, t3829: f64, t3889: f64, t4045: f64, t539: f64, t5644: f64, t5650: f64, t5652: f64, t6837: f64, t6840: f64, t73: f64, t73578: f64, t73991: f64, t74012: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t74024 = t9775 * t22263;
    let t74026 = t1412 * t6861;
    let t74029 = t2661 * t3992 * t74026 * t3938;
    let t74033 = t2661 * t3992 * t5608 * t5659;
    let t74037 = t2661 * t3992 * t22025 * t1399;
    let t74077 = -24.0_f64 * t1353 * t1412 * t21969 * t5650 + 3.0_f64 * t1394 * t539 * t73578 - 12.0_f64 * t22245 * t3889 * t5650 - 360.0_f64 * t3829 * t5650 * t73991 + 60.0_f64 * t3829 * t5650 * t74012 - 48.0_f64 * t5644 * t5652 * t73 + 120.0_f64 * t13902 * t22237 - 48.0_f64 * t13902 * t22240 - 24.0_f64 * t13902 * t22246 + 120.0_f64 * t13907 * t22229 + 6.0_f64 * t1392 * t22249 - 12.0_f64 * t4045 * t6837 + 3.0_f64 * t4045 * t6840;
    (t74024, t74026, t74029, t74033, t74037, t74077)
}

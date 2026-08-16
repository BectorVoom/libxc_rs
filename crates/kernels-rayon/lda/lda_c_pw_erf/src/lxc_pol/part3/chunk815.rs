//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 815/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk815(t1832: f64, t411: f64, t1568: f64, t756: f64, t1697: f64, t9: f64, t1664: f64, t1870: f64, t1871: f64, t3349: f64, t3351: f64, t5505: f64, t5517: f64, t5520: f64, t5560: f64, t5561: f64, t5562: f64, t5563: f64, t5641: f64) -> (f64, f64, f64, f64, f64) {
    let t5643 = t1832 * t411;
    let t5647 = t756 * t1568;
    let t5651 = t9 * t1697;
    let t5652 = t756 * t1664;
    let t5658 = -t5505 + t5517 + t5520 - 3.44851_f64 * t5641 + 10.34553_f64 * t1870 * t1871 * t5643 + 5.172765_f64 * t1870 * t1871 * t5647 - 20.69106_f64 * t1870 * t5651 * t5652 + t5560 + t5561 - t5562 - t5563 - 1.532671111111111_f64 * t3349 + 0.5747516666666667_f64 * t3351;
    (t5643, t5647, t5651, t5652, t5658)
}

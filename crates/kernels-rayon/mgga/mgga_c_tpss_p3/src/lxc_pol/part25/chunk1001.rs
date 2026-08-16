//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1001/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1001(t5343: f64, t72: f64, t732: f64, t10019: f64, t10028: f64, t12754: f64, t12757: f64, t12769: f64, t12780: f64, t13623: f64, t13624: f64, t13631: f64, t13637: f64, t13645: f64, t9954: f64, t9956: f64, t9972: f64, t9980: f64) -> (f64, f64) {
    let t13806 = t5343 * t72;
    let t13807 = t13806 * t732;
    let t13808 = 0.18311447306006545054e-3_f64 * t13807;
    let t13809 = t13623 - t9954 + t9956 + t13624 - t12754 - t12757 + t13631 - t12769 - t9972 - t13637 - t9980 + t13645 + t10019 + t12780 - t10028 - t13808;
    (t13808, t13809)
}

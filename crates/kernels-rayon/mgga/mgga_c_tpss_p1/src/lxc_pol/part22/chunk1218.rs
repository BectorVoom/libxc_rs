//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1218/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1218(t18009: f64, t18770: f64, t5572: f64, t5831: f64, t818: f64, t1805: f64, t2425: f64, t2161: f64, t18021: f64, t2162: f64, t226: f64, t782: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t18771 = t18770 * t18009;
    let t18775 = t5572 * t5831 * t818;
    let t18779 = t5572 * t1805 * t2425;
    let t18782 = t1805 * t2161;
    let t18784 = t18021 * t18782 * t2162;
    let t18788 = t5831 * t782 * t226;
    (t18771, t18775, t18779, t18782, t18784, t18788)
}

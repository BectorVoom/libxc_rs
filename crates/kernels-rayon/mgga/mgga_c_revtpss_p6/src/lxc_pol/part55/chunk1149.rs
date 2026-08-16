//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1149/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1149(t136: f64, t2457: f64, t8651: f64, t31837: f64, t93189: f64, t120000: f64, t32471: f64, t119816: f64, t1949: f64, t28425: f64, t25331: f64, t32481: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t121834 = t8651 * t136 * t2457;
    let t121836 = 0.50779446784275991476e-2_f64 * t93189 * t31837 * t121834;
    let t121838 = t120000 * t31837 * t32471;
    let t121840 = 0.39666484489654411541e-3_f64 * t119816;
    let t121846 = t28425 * t1949;
    let t121851 = 0.34270468708064099208e-1_f64 * t32481 * t25331;
    (t121834, t121836, t121838, t121840, t121846, t121851)
}

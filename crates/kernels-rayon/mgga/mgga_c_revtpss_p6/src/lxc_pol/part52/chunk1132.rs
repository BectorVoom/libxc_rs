//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1132/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1132(t121043: f64, t3985: f64, t8591: f64, t1385: f64, t240: f64, t843: f64, t31752: f64, t32197: f64, t8477: f64, t8705: f64, t9656: f64, t1419: f64, t31805: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t121044 = 0.34708173928447610098e-2_f64 * t121043;
    let t121045 = t8591 * t3985;
    let t121056 = t1385 * t843 * t240;
    let t121057 = t31752 * t121056;
    let t121058 = t121057 * t32197;
    let t121059 = 0.263521689745817692e-2_f64 * t121058;
    let t121076 = t8477 * t8705 * t9656;
    let t121099 = t31805 * t1419;
    (t121044, t121045, t121057, t121059, t121076, t121099)
}

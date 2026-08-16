//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1211/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1211(t127767: f64, t7060: f64, t786: f64, t122037: f64, t27341: f64, t103452: f64, t121991: f64, t121992: f64, t121993: f64, t121998: f64, t126345: f64, t126358: f64, t14587: f64, t1949: f64, t27206: f64, t28425: f64, t32426: f64, t32463: f64, t34054: f64) -> f64 {
    let t127827 = t786 * t127767 * t7060;
    let t127833 = t122037 * t27341;
    let t127841 = 0.7437465841810202164e-3_f64 * t126345 + t121991 - t121992 + 0.25389723392137995738e-1_f64 * t121993 + t121998 + 0.14456046980341999104e-1_f64 * t127827 + 0.34271842599061411569e1_f64 * t32463 * t103452 * t1949 * t14587 + 0.51405703062096148813e-1_f64 * t127833 - 0.3718732920905101082e-3_f64 * t126358 + 0.57119737665102352616e0_f64 * t32426 * t34054 - 0.11423947533020470523e1_f64 * t32463 * t28425 * t27206;
    t127841
}

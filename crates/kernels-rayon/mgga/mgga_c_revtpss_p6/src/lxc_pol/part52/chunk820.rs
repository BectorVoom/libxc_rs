//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 820/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk820(t8707: f64, t8708: f64, t2030: f64, t8576: f64, t8592: f64, t8702: f64, t8706: f64) -> (f64, f64) {
    let t8709 = t8707 * t8708;
    let t8713 = 0.56468933516960933999e-3_f64 * t8576 - 0.8673628188205199462e0_f64 * t8702 * t2030 + 0.57119737665102352616e0_f64 * t8706 * t8709 - 0.3718732920905101082e-3_f64 * t8592;
    (t8709, t8713)
}

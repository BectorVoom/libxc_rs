//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 622/1422 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk622(t371: f64, t482: f64, t676: f64, t481: f64, t1231: f64, t1256: f64, t225: f64, t3555: f64) -> (f64, f64, f64, f64) {
    let t3655 = t371 * t676 * t482;
    let t3657 = 0.47637797908966374413e-4_f64 * t481 * t3655;
    let t3658 = t1231 * t1256;
    let t3666 = t3555 * t225;
    (t3655, t3657, t3658, t3666)
}

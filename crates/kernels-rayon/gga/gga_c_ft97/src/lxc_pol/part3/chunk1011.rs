//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 1011/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk1011(t19020: f64, t2771: f64, t17744: f64, t4206: f64, t10595: f64, t14953: f64, t14955: f64, t14957: f64, t14999: f64, t15011: f64, t15014: f64, t15015: f64, t15025: f64, t15028: f64, t19640: f64, t19643: f64, t19646: f64, t19649: f64, t19651: f64, t19653: f64, t19656: f64, t19659: f64, t3139: f64, t462: f64) -> f64 {
    let t19662 = t2771 * t19020;
    let t19665 = t4206 * t17744;
    let t19668 = -t14953 - t14955 + t14957 - t14999 - 8.0_f64 / 9.0_f64 * t15011 + t15014 - 4.0_f64 / 9.0_f64 * t15015 - 8.0_f64 / 27.0_f64 * t15025 - t15028 - 10.0_f64 / 27.0_f64 * t462 * t19640 + 8.0_f64 / 9.0_f64 * t3139 * t19643 + 2.0_f64 / 3.0_f64 * t462 * t19646 - 2.0_f64 / 9.0_f64 * t19649 + t19651 / 3.0_f64 - 2.0_f64 / 3.0_f64 * t19653 - 4.0_f64 / 9.0_f64 * t10595 + t462 * t19656 / 3.0_f64 - 2.0_f64 / 3.0_f64 * t462 * t19659 - 2.0_f64 / 3.0_f64 * t462 * t19662 - 2.0_f64 * t462 * t19665;
    t19668
}

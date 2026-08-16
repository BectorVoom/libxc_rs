//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1041/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1041(t1937: f64, t29432: f64, t6993: f64, t7586: f64, t7316: f64, t8764: f64, t7239: f64, t32101: f64, t32102: f64, t32104: f64, t32107: f64, t32109: f64, t32112: f64, t32116: f64, t32825: f64, t671: f64, t8463: f64) -> f64 {
    let t32843 = t29432 * t1937;
    let t32845 = t7586 * t6993;
    let t32849 = t8764 * t7316;
    let t32850 = t8764 * t7239;
    let t32853 = -2.0_f64 * t32825 * t671 + t32101 - t32102 - 2.0_f64 * t32104 - t32107 - t32109 - t32112 - t32116 - 2.0_f64 * t32843 - 2.0_f64 * t32845 - t32849 + 3.0_f64 * t32850 - t8463;
    t32853
}

//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 513/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk513(t229: f64, t2483: f64, t595: f64, t970: f64, t637: f64, t406: f64, t959: f64, t410: f64, t697: f64, t898: f64, t60: f64, t955: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t2755 = t2483 * t229;
    let t2758 = t595 * t970;
    let t2759 = t2758 * t637;
    let t2761 = t406 * t959;
    let t2763 = t410 * t959;
    let t2765 = t898 * t697;
    let t2768 = t60 * t955;
    (t2755, t2758, t2759, t2761, t2763, t2765, t2768)
}

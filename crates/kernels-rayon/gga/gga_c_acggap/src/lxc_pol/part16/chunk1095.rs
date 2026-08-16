//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1095/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1095(t570: f64, t6175: f64, t5636: f64, t1745: f64, t2009: f64, t1988: f64, t9549: f64, t34296: f64, t34298: f64, t34305: f64, t34308: f64, t37012: f64, t39141: f64, t39143: f64, t39145: f64, t39147: f64, t39151: f64, t39155: f64, t39160: f64, t39162: f64, t39167: f64) -> f64 {
    let t39169 = t570 * t6175;
    let t39171 = t570 * t5636;
    let t39173 = t2009 * t1745;
    let t39176 = t1988 * t9549;
    let t39178 = -0.94344276868812456204e-3_f64 * t39141 + 0.68598428988911579156e-2_f64 * t39143 + 0.34299214494455789578e-2_f64 * t39145 - 0.17149607247227894789e-2_f64 * t39147 - 0.62896184579208304136e-3_f64 * t39151 - 0.7862023072401038017e-3_f64 * t39155 + 0.31448092289604152068e-3_f64 * t39160 - 0.64311027177104605458e-2_f64 * t39162 - 0.47172138434406228102e-3_f64 * t39167 - t39169 / 48.0_f64 - t39171 / 96.0_f64 - 0.40015750243531754507e-2_f64 * t39173 + t37012 + t34296 - t34298 + 0.62896184579208304135e-3_f64 * t34305 + t34308 - 0.10718504529517434243e-2_f64 * t39176;
    t39178
}

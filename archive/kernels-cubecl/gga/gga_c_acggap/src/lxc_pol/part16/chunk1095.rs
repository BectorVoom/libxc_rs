//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1095/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1095<F: Float>(t570: F, t6175: F, t5636: F, t1745: F, t2009: F, t1988: F, t9549: F, t34296: F, t34298: F, t34305: F, t34308: F, t37012: F, t39141: F, t39143: F, t39145: F, t39147: F, t39151: F, t39155: F, t39160: F, t39162: F, t39167: F) -> F {
    let t39169 = t570 * t6175;
    let t39171 = t570 * t5636;
    let t39173 = t2009 * t1745;
    let t39176 = t1988 * t9549;
    let t39178 = -F::cast_from(0.94344276868812456204e-3_f64) * t39141 + F::cast_from(0.68598428988911579156e-2_f64) * t39143 + F::cast_from(0.34299214494455789578e-2_f64) * t39145 - F::cast_from(0.17149607247227894789e-2_f64) * t39147 - F::cast_from(0.62896184579208304136e-3_f64) * t39151 - F::cast_from(0.7862023072401038017e-3_f64) * t39155 + F::cast_from(0.31448092289604152068e-3_f64) * t39160 - F::cast_from(0.64311027177104605458e-2_f64) * t39162 - F::cast_from(0.47172138434406228102e-3_f64) * t39167 - t39169 / F::cast_from(48.0_f64) - t39171 / F::cast_from(96.0_f64) - F::cast_from(0.40015750243531754507e-2_f64) * t39173 + t37012 + t34296 - t34298 + F::cast_from(0.62896184579208304135e-3_f64) * t34305 + t34308 - F::cast_from(0.10718504529517434243e-2_f64) * t39176;
    t39178
}

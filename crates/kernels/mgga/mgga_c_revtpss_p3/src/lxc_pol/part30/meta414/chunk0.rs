//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1552/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1552<F: Float>(t4628: F, t698: F, t15193: F, t930: F, t141: F, t15127: F, t15125: F, t15191: F, t11134: F, t11136: F, t11138: F, t11140: F, t11304: F, t15132: F, t15137: F, t15142: F, t15147: F, t15151: F, t15156: F, t15160: F, t15189: F, t15195: F) -> (F, F, F, F) {
    let t15197 = t698 * t4628;
    let t15198 = F::cast_from(0.11038e0_f64) * t15197;
    let t15199 = t930 * t15193;
    let t15200 = t141 * t15199;
    let t15209 = F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t15127;
    let t15210 = F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t15125;
    let t15211 = F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t15191;
    let t15220 = -t11304 - F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t11134 + F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t11136 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t11138 + t11140 / F::cast_from(9.0_f64) - F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t15189 + t15209 - t15210 + t15211 - F::cast_from(10.0_f64) / F::cast_from(27.0_f64) * t15142 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t15156 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t15132 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t15137 - F::cast_from(2.0_f64) * t15160 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t15147 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t15151 - t15195 / F::cast_from(3.0_f64);
    (t15197, t15198, t15200, t15220)
}

//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1229/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk1229<F: Float>(t4628: F, t698: F, t15193: F, t930: F, t141: F, t15127: F, t15125: F, t15191: F, t11134: F, t11136: F, t11138: F, t11140: F, t11304: F, t15132: F, t15137: F, t15142: F, t15147: F, t15151: F, t15156: F, t15160: F, t15189: F, t15195: F) -> (F, F, F, F) {
    let t15197 = t698 * t4628;
    let t15198 = F::new(0.11038e0) * t15197;
    let t15199 = t930 * t15193;
    let t15200 = t141 * t15199;
    let t15209 = F::new(4.0) / F::new(27.0) * t15127;
    let t15210 = F::new(4.0) / F::new(9.0) * t15125;
    let t15211 = F::new(2.0) / F::new(9.0) * t15191;
    let t15220 = -t11304 - F::new(8.0) / F::new(27.0) * t11134 + F::new(2.0) / F::new(27.0) * t11136 - F::new(2.0) / F::new(9.0) * t11138 + t11140 / F::new(9.0) - F::new(4.0) / F::new(27.0) * t15189 + t15209 - t15210 + t15211 - F::new(10.0) / F::new(27.0) * t15142 + F::new(4.0) / F::new(3.0) * t15156 - F::new(4.0) / F::new(9.0) * t15132 - F::new(2.0) / F::new(9.0) * t15137 - F::new(2.0) * t15160 + F::new(4.0) / F::new(3.0) * t15147 + F::new(2.0) / F::new(3.0) * t15151 - t15195 / F::new(3.0);
    (t15197, t15198, t15200, t15220)
}

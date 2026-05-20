//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3604/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3604<F: Float>(t12254: F, t141: F, t68265: F, t43881: F, t68253: F, t68255: F, t68257: F, t68262: F, t68267: F, t68271: F, t68275: F, t68277: F, t68282: F, t68287: F, t68292: F) -> (F, F) {
    let t68402 = t141 * t12254 * t68265;
    let t68415 = F::new(4.0) / F::new(3.0) * t68253 + F::new(4.0) / F::new(27.0) * t68255 - F::new(8.0) / F::new(81.0) * t68257 + t43881 - F::new(20.0) / F::new(81.0) * t68262 + F::new(10.0) / F::new(27.0) * t68267 + F::new(8.0) * t68271 + F::new(4.0) / F::new(3.0) * t68275 - F::new(4.0) / F::new(9.0) * t68277 - F::new(4.0) / F::new(9.0) * t68282 - F::new(2.0) / F::new(9.0) * t68287 - F::new(4.0) / F::new(3.0) * t68292;
    (t68402, t68415)
}

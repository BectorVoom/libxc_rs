//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 855/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk855<F: Float>(t31139: F, t544: F, t986: F, t2386: F, t10525: F, t10526: F, t41965: F, t3177: F, t35091: F, t9272: F, t204: F, t41726: F, t587: F) -> (F, F, F, F) {
    let t42219 = t544 * t31139 * t986;
    let t42221 = F::new(0.25025342966295298669e1) * t42219 * t2386;
    let t42224 = F::new(0.21450293971110256001e1) * t10525 * t10526 * t41965;
    let t42226 = t9272 * t35091 * t3177;
    let t42227 = F::new(0.11502877786176224903e1) * t42226;
    let t42230 = F::new(0.18404604457881959845e2) * t587 * t204 * t41726;
    (t42221, t42224, t42227, t42230)
}

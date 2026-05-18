//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 855/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk855<F: Float>(t45228: F, t44713: F, t4820: F, t7513: F, t11780: F, t23000: F, t2617: F, t2679: F, t9805: F, t23477: F, t44708: F, t43432: F) -> (F, F, F, F, F, F) {
    let t45229 = F::new(0.19171462976960374838e0) * t45228;
    let t45232 = F::new(0.79445533226334281487e-1) * t7513 * t4820 * t44713;
    let t45234 = t23000 * t11780 * t2617;
    let t45238 = t9805 * t11780 * t2679;
    let t45242 = F::new(0.23833659967900284446e0) * t23477 * t4820 * t44708;
    let t45243 = F::new(0.11916829983950142223e0) * t43432;
    (t45229, t45232, t45234, t45238, t45242, t45243)
}

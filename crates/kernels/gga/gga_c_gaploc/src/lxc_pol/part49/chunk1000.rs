//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 1000/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk1000<F: Float>(t40758: F, t13185: F, t7129: F, t13217: F, t10673: F, t2508: F, t954: F, t13191: F, t7137: F, t33285: F, t7659: F, t3276: F, t8682: F) -> (F, F, F, F, F, F, F) {
    let t43157 = F::new(0.64087718584518535698e-3) * t40758;
    let t43166 = F::new(0.53833683610995569986e-1) * t7129 * t13185;
    let t43168 = F::new(0.46143157380853345701e-1) * t7129 * t13217;
    let t43170 = t2508 * t954 * t10673;
    let t43173 = F::new(0.12304841968227558854e0) * t7137 * t13191;
    let t43175 = t2508 * t33285 * t7659;
    let t43179 = F::new(0.11535789345213336425e0) * t2508 * t3276 * t8682;
    (t43157, t43166, t43168, t43170, t43173, t43175, t43179)
}

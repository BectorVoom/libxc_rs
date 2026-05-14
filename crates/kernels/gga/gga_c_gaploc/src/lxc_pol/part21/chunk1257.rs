//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1257/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1257<F: Float>(t12236: F, t1402: F, t2033: F, t28406: F, t28407: F, t28409: F, t28415: F, t28419: F, t28421: F, t33130: F, t33132: F, t33134: F, t33136: F, t33145: F, t33151: F, t33154: F, t33158: F, t33164: F) -> (F,) {
    let t39039 = -t33130 - t33132 + t33134 + t33136 - t33145 - 0.92686455430723328401e-1 * t2033 * t1402 * t12236 + t33151 - t33154 - t28406 - 0.51123901271894332903e1 * t28407 + 0.30674340763136599742e1 * t28409 - t28415 + t33158 - t33164 - t28419 - t28421;
    (t39039,)
}

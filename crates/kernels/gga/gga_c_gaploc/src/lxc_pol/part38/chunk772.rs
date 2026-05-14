//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 772/861 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk772<F: Float>(t43412: F, t43416: F, t43363: F, t43421: F, t45170: F, t45174: F, t45176: F, t45177: F, t45178: F, t45179: F, t45180: F, t45183: F, t45187: F, t45188: F, t45192: F, t45193: F, t45194: F, t45195: F, t45197: F) -> (F,) {
    let t45199 = 0.15337170381568299871e1 * t43412;
    let t45200 = 0.15337170381568299871e1 * t43416;
    let t45202 = t45170 - t45174 - 0.38342925953920749677e1 * t43363 - t45176 - t45177 - t45178 + t45179 - t45180 - t45183 + t45187 + t45188 + t45192 - t45193 - t45194 + t45195 + 0.38342925953920749677e0 * t45197 - t45199 - t45200 - 0.23005755572352449806e1 * t43421;
    (t45202,)
}

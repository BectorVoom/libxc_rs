//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 1105/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk1105<F: Float>(t2684: F, t2685: F, t47143: F, t12213: F, t2464: F, t2465: F, t13851: F, t2013: F, t43393: F, t43398: F, t43401: F, t43404: F, t43408: F, t43409: F, t47133: F, t47137: F, t47140: F) -> F {
    let t47145 = t2684 * t2685 * t47143;
    let t47149 = t2684 * t2464 * t2465 * t12213;
    let t47151 = t2013 * t13851;
    let t47153 = F::cast_from(0.15337170381568299871e1_f64) * t43393 + t43398 - t43401 - t43404 + t43408 + F::cast_from(0.15337170381568299871e1_f64) * t47133 - F::cast_from(0.25561950635947166451e1_f64) * t47137 + F::cast_from(0.25561950635947166451e0_f64) * t47140 + F::cast_from(0.19171462976960374838e0_f64) * t47145 - F::cast_from(0.42603251059911944084e-1_f64) * t47149 - F::cast_from(0.19171462976960374838e0_f64) * t47151 + t43409;
    t47153
}

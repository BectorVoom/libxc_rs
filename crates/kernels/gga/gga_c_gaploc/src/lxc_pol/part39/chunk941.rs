//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 941/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk941<F: Float>(t13851: F, t2013: F, t43393: F, t43398: F, t43401: F, t43404: F, t43408: F, t43409: F, t47133: F, t47137: F, t47140: F, t47145: F, t47149: F, t40986: F, t40989: F, t13883: F, t1991: F, t590: F) -> (F, F, F, F) {
    let t47151 = t2013 * t13851;
    let t47153 = 0.15337170381568299871e1 * t43393 + t43398 - t43401 - t43404 + t43408 + 0.15337170381568299871e1 * t47133 - 0.25561950635947166451e1 * t47137 + 0.25561950635947166451e0 * t47140 + 0.19171462976960374838e0 * t47145 - 0.42603251059911944084e-1 * t47149 - 0.19171462976960374838e0 * t47151 + t43409;
    let t47155 = 0.38342925953920749677e0 * t40986;
    let t47157 = 0.72851559312449424385e1 * t40989;
    let t47160 = 0.51123901271894332902e0 * t1991 * t13883 * t590;
    (t47153, t47155, t47157, t47160)
}

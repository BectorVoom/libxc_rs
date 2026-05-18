//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1194/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1194<F: Float>(t10894: F, t3086: F, t30285: F, t3332: F, t6165: F, t11646: F, t25983: F, t11649: F, t30792: F, t12529: F, t6395: F, t3281: F, t9273: F) -> (F, F, F, F, F, F) {
    let t43266 = t10894 * t3086;
    let t43269 = t6165 * t3332 * t30285;
    let t43271 = t25983 * t11646;
    let t43273 = t30792 * t11649;
    let t43275 = t6395 * t12529;
    let t43277 = t3281 * t9273;
    (t43266, t43269, t43271, t43273, t43275, t43277)
}

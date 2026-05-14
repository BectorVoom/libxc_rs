//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 919/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk919<F: Float>(t236: F, t615: F, t1981: F, t41799: F, t676: F, t46832: F, t7473: F, t7478: F, t40702: F, t8571: F, t40081: F, t46434: F, t7198: F, t46438: F, t7204: F, t10247: F, t10248: F, t10249: F, t42369: F, t42372: F, t42373: F, t42374: F, t42375: F, t42376: F, t8350: F, t8356: F) -> (F, F, F, F, F, F, F) {
    let t48033 = t236 * t615;
    let t48036 = t41799 * t1981 * t676 * t48033;
    let t48038 = t46832 * t7473;
    let t48039 = t48038 * t7478;
    let t48041 = t8571 * t40702;
    let t48043 = t8571 * t40081;
    let t48047 = t7198 * t46434;
    let t48049 = t7204 * t46438;
    let t48102 = -t10247 - t10248 - t10249 + t42369 - t42372 - t42373 - 0.12195059916630011325e-2 * t8350 - t42374 - 0.12195059916630011325e-2 * t8356 - t42375 - t42376;
    (t48036, t48039, t48041, t48043, t48047, t48049, t48102)
}

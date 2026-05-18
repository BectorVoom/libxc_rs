//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1060/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1060<F: Float>(t46438: F, t7204: F, t10247: F, t10248: F, t10249: F, t42369: F, t42372: F, t42373: F, t42374: F, t42375: F, t42376: F, t8350: F, t8356: F) -> (F, F) {
    let t48049 = t7204 * t46438;
    let t48102 = -t10247 - t10248 - t10249 + t42369 - t42372 - t42373 - F::new(0.12195059916630011325e-2) * t8350 - t42374 - F::new(0.12195059916630011325e-2) * t8356 - t42375 - t42376;
    (t48049, t48102)
}

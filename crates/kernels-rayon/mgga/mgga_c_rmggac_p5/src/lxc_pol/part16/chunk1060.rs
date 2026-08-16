//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1060/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1060(t46438: f64, t7204: f64, t10247: f64, t10248: f64, t10249: f64, t42369: f64, t42372: f64, t42373: f64, t42374: f64, t42375: f64, t42376: f64, t8350: f64, t8356: f64) -> (f64, f64) {
    let t48049 = t7204 * t46438;
    let t48102 = -t10247 - t10248 - t10249 + t42369 - t42372 - t42373 - 0.12195059916630011325e-2_f64 * t8350 - t42374 - 0.12195059916630011325e-2_f64 * t8356 - t42375 - t42376;
    (t48049, t48102)
}

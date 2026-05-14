//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1101/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1101<F: Float>(t35874: F, t35876: F, t37743: F, t37744: F, t40251: F, t40253: F, t40255: F, t40257: F, t40260: F, t40262: F, t40264: F, t40268: F, t40270: F, t40272: F, t40274: F, t40277: F, t40280: F, t40283: F) -> (F,) {
    let t42023 = 0.31448092289604152069e-2 * t40251 + 0.13719685797782315831e-1 * t40253 - 0.13719685797782315831e-1 * t40255 + 0.68598428988911579156e-2 * t40257 - t37743 - t37744 - 0.1528125e-1 * t40260 + 0.17149607247227894789e-2 * t40262 + 0.17149607247227894789e-2 * t40264 - 0.94344276868812456207e-3 * t40268 - 0.51448821741683684366e-2 * t40270 + 0.68598428988911579156e-2 * t40272 - 0.68598428988911579156e-2 * t40274 + 0.85748036236139473944e-3 * t40277 - 0.51448821741683684368e-2 * t35874 - 0.42874018118069736972e-3 * t40280 + 0.75475421495049964964e-2 * t35876 - 0.21437009059034868486e-2 * t40283;
    (t42023,)
}

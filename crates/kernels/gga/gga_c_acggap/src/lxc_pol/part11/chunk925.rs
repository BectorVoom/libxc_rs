//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 925/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk925<F: Float>(t34222: F, t34223: F, t34225: F, t34227: F, t34229: F, t34233: F, t34237: F, t34240: F, t34242: F, t34246: F, t34251: F, t34255: F, t34259: F, t34263: F, t34265: F, t34269: F, t34271: F, t34273: F) -> (F,) {
    let t34275 = -t34222 + 0.68598428988911579156e-2 * t34223 - 0.34299214494455789578e-2 * t34225 + 0.51448821741683684366e-2 * t34227 + 0.25724410870841842183e-2 * t34229 - 0.18868855373762491241e-2 * t34233 + 0.21437009059034868486e-3 * t34237 - t34240 + 0.31448092289604152068e-3 * t34242 + 0.94344276868812456204e-3 * t34246 + 0.94344276868812456205e-2 * t34251 - 0.42874018118069736972e-3 * t34255 + 0.94344276868812456204e-2 * t34259 - 0.62896184579208304136e-3 * t34263 + 0.85748036236139473944e-3 * t34265 - 0.37737710747524982482e-2 * t34269 - 0.85748036236139473944e-3 * t34271 - 0.40015750243531754508e-2 * t34273;
    (t34275,)
}

//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1038/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1038<F: Float>(t34222: F, t34223: F, t34225: F, t34227: F, t34229: F, t34233: F, t34237: F, t34240: F, t34242: F, t34246: F, t34251: F, t34255: F, t34259: F, t34263: F, t34265: F, t34269: F, t34271: F, t34273: F) -> F {
    let t34275 = -t34222 + F::cast_from(0.68598428988911579156e-2_f64) * t34223 - F::cast_from(0.34299214494455789578e-2_f64) * t34225 + F::cast_from(0.51448821741683684366e-2_f64) * t34227 + F::cast_from(0.25724410870841842183e-2_f64) * t34229 - F::cast_from(0.18868855373762491241e-2_f64) * t34233 + F::cast_from(0.21437009059034868486e-3_f64) * t34237 - t34240 + F::cast_from(0.31448092289604152068e-3_f64) * t34242 + F::cast_from(0.94344276868812456204e-3_f64) * t34246 + F::cast_from(0.94344276868812456205e-2_f64) * t34251 - F::cast_from(0.42874018118069736972e-3_f64) * t34255 + F::cast_from(0.94344276868812456204e-2_f64) * t34259 - F::cast_from(0.62896184579208304136e-3_f64) * t34263 + F::cast_from(0.85748036236139473944e-3_f64) * t34265 - F::cast_from(0.37737710747524982482e-2_f64) * t34269 - F::cast_from(0.85748036236139473944e-3_f64) * t34271 - F::cast_from(0.40015750243531754508e-2_f64) * t34273;
    t34275
}

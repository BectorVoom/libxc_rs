//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 1146/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk1146<F: Float>(t36009: F, t36011: F, t36013: F, t36017: F, t36020: F, t36022: F, t36025: F, t36028: F, t36030: F, t36034: F, t36037: F, t36040: F, t36042: F, t36044: F, t338: F, t36144: F, t36158: F, t36173: F, t36188: F, t36204: F, t36218: F, t36233: F) -> (F,) {
    let t36248 = -0.32829531147150437834e-4 * t36009 + 0.14226130163765189728e-3 * t36011 + 0.14226130163765189728e-3 * t36013 + 0.29357452990051769742e-5 * t36017 - 0.16414765573575218917e-4 * t36020 + 0.16414765573575218917e-4 * t36022 + 0.46971924784082831588e-5 * t36025 + 0.32829531147150437834e-4 * t36028 - 0.21920231565905321408e-4 * t36030 + 0.32829531147150437834e-4 * t36034 - 0.16414765573575218917e-4 * t36037 - 0.22798285518854470718e-6 * t36040 + 0.37936347103373839275e-3 * t36042 - 0.21658371242911747182e-5 * t36044;
    let t36252 = (t36144 + t36158 + t36173 + t36188 + t36204 + t36218 + t36233 + t36248) * t338;
    (t36252,)
}

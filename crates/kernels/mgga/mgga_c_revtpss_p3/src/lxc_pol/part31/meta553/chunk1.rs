//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1957/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1957<F: Float>(t6884: F, t7252: F, t25983: F, t6864: F, t26003: F, t26011: F, t26013: F, t26022: F, t27921: F, t27953: F, t28873: F, t28874: F, t28885: F) -> F {
    let t30048 = t7252 * t6884;
    let t30050 = t25983 * t6864;
    let t30054 = t26003 - t26011 - t30048 / F::new(48.0) + t28885 + F::cast_from(0.85748036236139473944e-3_f64) * t30050 + t26013 + t26022 - F::cast_from(0.50820002809285328226e-4_f64) * t27953 + t28873 + t28874 + F::cast_from(0.40015750243531754508e-2_f64) * t27921;
    t30054
}

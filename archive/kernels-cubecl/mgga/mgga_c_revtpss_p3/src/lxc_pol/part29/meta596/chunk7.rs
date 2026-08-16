//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 2010/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2010<F: Float>(t98937: F, t98949: F, t92952: F, t92956: F, t98940: F, t98943: F, t98945: F, t98947: F, t98951: F, t98953: F, t98955: F, t98957: F) -> F {
    let t103247 = F::cast_from(0.16006300097412701803e-1_f64) * t98937;
    let t103254 = F::cast_from(0.32012600194825403606e-1_f64) * t98949;
    let t103259 = -t103247 - F::cast_from(0.32012600194825403606e-1_f64) * t92952 - F::cast_from(0.85748036236139473944e-3_f64) * t98940 + F::cast_from(0.4065600224742826258e-3_f64) * t92956 - F::cast_from(0.17149607247227894789e-1_f64) * t98943 + F::cast_from(0.34299214494455789578e-2_f64) * t98945 - F::cast_from(0.68598428988911579156e-2_f64) * t98947 - t103254 - F::cast_from(0.34299214494455789578e-1_f64) * t98951 - F::cast_from(0.85748036236139473944e-3_f64) * t98953 - F::cast_from(0.13719685797782315831e-1_f64) * t98955 + F::cast_from(0.68598428988911579156e-2_f64) * t98957;
    t103259
}

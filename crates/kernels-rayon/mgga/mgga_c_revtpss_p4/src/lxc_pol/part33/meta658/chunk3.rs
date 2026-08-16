//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2119/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2119(t18423: f64, t25234: f64, t25222: f64, t5993: f64, t103269: f64, t103270: f64, t103285: f64, t92989: f64, t92991: f64, t98984: f64, t98992: f64, t99001: f64, t99002: f64, t99007: f64) -> f64 {
    let t106022 = t25234 * t18423;
    let t106024 = t25222 * t5993;
    let t106028 = 0.50820002809285328226e-3_f64 * t106022 - 0.40015750243531754508e-1_f64 * t106024 - t103269 + t103270 + t98984 + t98992 - t99001 + 0.54208002996571016775e-3_f64 * t99002 - t92989 + 0.2032800112371413129e-4_f64 * t92991 + t99007 - t103285;
    t106028
}

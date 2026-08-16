//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 117/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk117(t101: f64, t125: f64, t143: f64, t145: f64, t148: f64, t153: f64, t156: f64, t163: f64, t164: f64, t168: f64, t169: f64, t171: f64, t234: f64, t242: f64, t245: f64, t270: f64, t274: f64, t279: f64, t281: f64, t286: f64, t289: f64, t296: f64, t299: f64, t301: f64) -> f64 {
    let t305 = t101 * t143 + (-0.31505407223141117834e-1_f64 * t148 * t164 - 0.53884053046145740922e-2_f64 * t169 * t171 * t234 * t163) * t125 + (-0.83762820535504401876e-1_f64 * t148 * t242 - 0.11938374665504764976e-1_f64 * t168 * t245 * t270 + 0.42708890021612718669e0_f64 * t153 * t156 * t274) * t279 - 0.11974234010254609094e-1_f64 * t281 * t286 + (-0.31835665774679373271e-1_f64 * t169 * t289 * t242 + 0.533250677421793803e-1_f64 * t145 * t274) * t296 + 0.20267214298646782767e-1_f64 * t169 * t299 * t274 * t301;
    t305
}

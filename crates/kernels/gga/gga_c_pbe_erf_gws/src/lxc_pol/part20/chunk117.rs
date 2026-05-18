//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 117/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk117<F: Float>(t101: F, t125: F, t143: F, t145: F, t148: F, t153: F, t156: F, t163: F, t164: F, t168: F, t169: F, t171: F, t234: F, t242: F, t245: F, t270: F, t274: F, t279: F, t281: F, t286: F, t289: F, t296: F, t299: F, t301: F) -> F {
    let t305 = t101 * t143 + (-F::new(0.31505407223141117834e-1) * t148 * t164 - F::new(0.53884053046145740922e-2) * t169 * t171 * t234 * t163) * t125 + (-F::new(0.83762820535504401876e-1) * t148 * t242 - F::new(0.11938374665504764976e-1) * t168 * t245 * t270 + F::new(0.42708890021612718669e0) * t153 * t156 * t274) * t279 - F::new(0.11974234010254609094e-1) * t281 * t286 + (-F::new(0.31835665774679373271e-1) * t169 * t289 * t242 + F::new(0.533250677421793803e-1) * t145 * t274) * t296 + F::new(0.20267214298646782767e-1) * t169 * t299 * t274 * t301;
    t305
}

//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 938/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk938<F: Float>(t285: F, t3013: F, t545: F, t39: F, t991: F, t159: F, t2522: F, t532: F, t143: F, t1501: F, t169: F, t279: F, t281: F, t2857: F, t2922: F, t299: F, t301: F, t475: F, t481: F, t526: F, t8038: F, t8061: F, t8075: F, t8102: F, t8108: F, t8112: F, t8258: F, t8261: F, t8267: F, t8270: F, t8275: F) -> F {
    let t8277 = t3013 * t545 * t285;
    let t8279 = t39 * t991;
    let t8281 = t8279 * t159 * t285;
    let t8287 = t532 * t2522;
    let t8290 = F::new(0.58113483035773838734e-3) * t8287 * t159 * t285;
    let t8291 = (t8061 + t8075) * t279 + F::new(3.0) * t475 * t143 * t8102 + F::new(2.0) * t2922 * t1501 + F::new(12.0) * t2857 * t8108 * t481 + F::new(6.0) * t2857 * t8112 + t8258 * t526 - F::new(0.11974234010254609094e-1) * t281 * t8261 - t8267 - F::new(0.11974234010254609094e-1) * t8270 - t8275 - F::new(0.58113483035773838734e-3) * t8277 + F::new(0.13559812708347229038e-2) * t8281 + F::new(0.20267214298646782767e-1) * t169 * t299 * t8038 * t301 - t8290;
    t8291
}

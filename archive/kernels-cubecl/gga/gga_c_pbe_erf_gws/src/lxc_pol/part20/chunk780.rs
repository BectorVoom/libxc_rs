//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 780/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk780<F: Float>(t2030: F, t520: F, t39: F, t535: F, t159: F, t285: F, t169: F, t301: F, t366: F, t745: F, t1457: F, t545: F) -> (F, F, F, F) {
    let t5660 = t2030 * t520;
    let t5668 = t39 * t535;
    let t5670 = t5668 * t159 * t285;
    let t5674 = t169 * t366 * t745 * t301;
    let t5690 = t1457 * t545 * t285;
    (t5660, t5670, t5674, t5690)
}

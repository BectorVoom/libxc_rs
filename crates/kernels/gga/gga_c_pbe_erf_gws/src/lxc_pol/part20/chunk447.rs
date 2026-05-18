//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 447/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk447<F: Float>(t1697: F, t219: F, t1764: F, t197: F, t720: F, t723: F, t156: F, t254: F, t252: F, t707: F, t719: F, t256: F) -> (F, F, F, F, F, F, F) {
    let t1891 = t219 * t1697;
    let t1896 = t197 * t1764;
    let t1902 = F::new(4.0) / F::new(9.0) * t720 * t723;
    let t1903 = t254 * t156;
    let t1905 = F::new(2.0) / F::new(27.0) * t252 * t1903;
    let t1910 = t707 * t719;
    let t1911 = t1910 * t256;
    (t1891, t1896, t1902, t1903, t1905, t1910, t1911)
}

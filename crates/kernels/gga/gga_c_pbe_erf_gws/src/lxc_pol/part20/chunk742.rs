//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 742/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk742<F: Float>(t1477: F, t328: F, t824: F, t822: F, t833: F, t2242: F, t941: F, t2200: F, t329: F, t340: F, t847: F, t2306: F, t2365: F) -> (F, F, F, F, F, F, F) {
    let t4422 = t328 * t1477;
    let t4423 = t824 * t4422;
    let t4424 = t822 * t4423;
    let t4425 = t4424 * t833;
    let t4430 = t2242 * t941;
    let t4442 = t329 * t2200 * t340;
    let t4443 = t4442 * t847;
    let t4473 = t2306 * t2365;
    (t4423, t4424, t4425, t4430, t4442, t4443, t4473)
}

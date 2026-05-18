//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 819/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk819<F: Float>(t2571: F, t4934: F, t1620: F, t219: F, t2591: F, t649: F, t1639: F, t331: F, t1621: F, t1791: F, t21: F, t5589: F) -> (F, F, F, F, F, F) {
    let t7191 = t4934 * t2571;
    let t7193 = F::new(32.0) / F::new(135.0) * t1620 * t7191;
    let t7194 = t2591 * t219;
    let t7199 = t2591 * t649;
    let t7209 = t331 * t1639;
    let t7210 = t7209 * t219;
    let t7216 = t1621 * t1791;
    let t7236 = t21 * t5589;
    (t7193, t7194, t7199, t7210, t7216, t7236)
}

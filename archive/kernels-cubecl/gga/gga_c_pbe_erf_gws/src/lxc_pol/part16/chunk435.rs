//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 435/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk435<F: Float>(t1642: F, t219: F, t1413: F, t1640: F, t639: F, t578: F, t586: F) -> (F, F, F, F) {
    let t1643 = t219 * t1642;
    let t1644 = t1643 * t1413;
    let t1645 = t1640 * t1644;
    let t1647 = F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t639 * t1645;
    let t1648 = t578 * t586;
    (t1644, t1645, t1647, t1648)
}

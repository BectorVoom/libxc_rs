//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 460/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk460<F: Float>(t562: F, t572: F, t418: F, t1821: F, t1820: F, t590: F, t597: F) -> (F, F, F, F) {
    let t1822 = t562 * t572;
    let t1823 = t1822 * t418;
    let t1824 = t1821 * t1823;
    let t1826 = F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t1820 * t1824;
    let t1827 = t590 * t597;
    (t1823, t1824, t1826, t1827)
}

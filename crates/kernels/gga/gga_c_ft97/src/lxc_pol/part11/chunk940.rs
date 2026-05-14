//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 940/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk940<F: Float>(t39370: F, t666: F, t669: F, t89: F, t193: F, t2373: F, t2459: F, t7514: F, t3704: F, t670: F, t41932: F, t41935: F, t41938: F, t41942: F, t41947: F, t41951: F, t41954: F, t41958: F, t41960: F, t41964: F, t41969: F) -> (F, F, F, F) {
    let t41973 = t89 * t666 * t669 * t39370;
    let t41978 = t89 * t193 * t7514 * t2373 * t2459;
    let t41981 = t89 * t3704 * t670;
    let t41982 = 56.0 / 243.0 * t41981;
    let t41983 = 2.0 / 9.0 * t41932 + 4.0 / 9.0 * t41935 - 4.0 / 27.0 * t41938 + 4.0 / 3.0 * t41942 + 2.0 / 9.0 * t41947 + t41951 - t41954 - t41958 - 8.0 / 27.0 * t41960 + 20.0 / 243.0 * t41964 + 20.0 / 27.0 * t41969 - t41973 / 18.0 - 6.0 * t41978 + t41982;
    (t41973, t41978, t41981, t41983)
}

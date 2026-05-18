//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 860/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk860<F: Float>(t13510: F, t905: F, t11975: F, t11944: F, t13485: F, t13486: F, t13488: F, t13493: F, t13498: F, t13500: F, t13503: F, t13505: F, t13507: F, t6592: F, t6597: F, t902: F, t929: F) -> (F, F, F) {
    let t13511 = t905 * t13510;
    let t13514 = F::new(7.0) / F::new(96.0) * t11975;
    let t13515 = t13485 - t13486 - t13488 - t13493 - F::new(35.0) / F::new(384.0) * t11944 + t13498 - F::new(5.0) / F::new(128.0) * t929 * t13500 + t13503 - t6592 - t6597 - t13505 - t929 * t13507 / F::new(768.0) + t902 * t13511 / F::new(384.0) - t13514;
    (t13511, t13514, t13515)
}

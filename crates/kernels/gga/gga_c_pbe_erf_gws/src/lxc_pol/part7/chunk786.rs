//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 786/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk786<F: Float>(t4806: F, t4814: F, t4687: F, t4710: F, t4713: F, t4717: F, t4825: F, t4810: F, t4817: F, t4819: F, t4821: F, t4823: F, t4827: F, t4830: F, t4833: F, t6850: F, t6856: F) -> (F,) {
    let t16366 = 0.14035736153892489771e2 * t4806;
    let t16368 = 0.22787712934626154593e-2 * t4814;
    let t16369 = 0.4274e0 * t4687;
    let t16370 = 0.28493333333333333333e0 * t4710;
    let t16371 = 0.2137e0 * t4713;
    let t16372 = 0.34366858576436911004e1 * t4717;
    let t16379 = 240.0 * t4825;
    let t16383 = t16366 + 0.29298488058805055905e-2 * t4810 - t16368 + t16369 + t16370 - t16371 - t16372 - 0.21973866044103791929e-2 * t4817 + 36.0 * t6850 + 8.0 * t6856 + 96.0 * t4819 - 96.0 * t4821 + 48.0 * t4823 + t16379 - 384.0 * t4827 + 240.0 * t4830 + 4.0 * t4833;
    (t16383,)
}

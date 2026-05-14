//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 884/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk884<F: Float>(t10049: F, t1243: F, t10052: F, t10074: F, t10077: F, t3656: F, t542: F, t496: F, t1251: F, t1508: F, t3652: F, t3660: F, t1552: F, t3665: F, t3668: F, t133: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t33965 = t10049 * t1243;
    let t33967 = t10052 * t1243;
    let t33973 = t10074 * t1243;
    let t33975 = t10077 * t1243;
    let t34038 = t542 * t3656;
    let t34039 = t496 * t34038;
    let t34045 = t1508 * t3652 * t1251;
    let t34080 = t542 * t3660;
    let t34081 = t496 * t34080;
    let t34084 = t1552 * t3665 * t1251;
    let t34087 = t1552 * t3668 * t1251;
    let t34158 = t133 * t34038;
    (t33965, t33967, t33973, t33975, t34039, t34045, t34080, t34081, t34084, t34087, t34158)
}

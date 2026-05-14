//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 928/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk928<F: Float>(t1105: F, t353: F, t3722: F, t4386: F, t13635: F, t2246: F, t13606: F, t2376: F, t829: F, t830: F, t11806: F, t337: F, t6560: F, t3772: F, t816: F, t13173: F, t2133: F) -> (F, F, F, F, F, F) {
    let t44149 = t4386 * t353 * t3722 * t1105;
    let t44158 = t2246 * t13635;
    let t44186 = t2376 * t13606;
    let t44188 = t829 * t830 * t44186;
    let t44213 = t11806 * t1105;
    let t44215 = t6560 * t337 * t44213;
    let t44220 = t816 * t3772;
    let t44230 = t13173 * t2133;
    (t44149, t44158, t44188, t44215, t44220, t44230)
}

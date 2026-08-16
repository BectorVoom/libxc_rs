//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1099/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1099<F: Float>(t4417: F, t810: F, t2370: F, t830: F, t2373: F, t4474: F, t2379: F, t4424: F, t2100: F, t2395: F, t829: F, t2367: F, t4402: F) -> (F, F, F, F, F) {
    let t19670 = t4417 * t810;
    let t19672 = t2370 * t830 * t19670;
    let t19677 = t4474 * t2373;
    let t19679 = t4424 * t2379;
    let t19683 = t829 * t830 * t2395 * t2100;
    let t19691 = t2367 * t4402;
    (t19672, t19677, t19679, t19683, t19691)
}

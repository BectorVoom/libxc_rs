//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1140/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1140<F: Float>(t13984: F, t14657: F, t3111: F, t3950: F, t850: F, t833: F, t1123: F, t13815: F, t2397: F, t4127: F, t2249: F, t904: F) -> (F, F, F, F, F, F, F) {
    let t14658 = t14657 * t13984;
    let t14673 = t850 * t3111 * t3950;
    let t14674 = t14673 * t833;
    let t14677 = t850 * t1123 * t13815;
    let t14678 = t14677 * t833;
    let t14680 = t4127 * t2397;
    let t14682 = t904 * t2249;
    (t14658, t14673, t14674, t14677, t14678, t14680, t14682)
}

//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1079/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1079<F: Float>(t13808: F, t3976: F, t2113: F, t3950: F, t850: F, t833: F, t331: F, t745: F, t851: F, t1192: F, t2182: F, t2376: F, t2409: F) -> (F, F, F, F, F, F, F, F, F) {
    let t13809 = t13808 * t3976;
    let t13810 = F::cast_from(7.0_f64) / F::cast_from(1152.0_f64) * t13809;
    let t13812 = t850 * t2113 * t3950;
    let t13813 = t13812 * t833;
    let t13815 = t745 * t331;
    let t13817 = t850 * t851 * t13815;
    let t13818 = t13817 * t833;
    let t13820 = t1192 * t2182;
    let t13822 = t2409 * t2376 * t13820;
    (t13809, t13810, t13812, t13813, t13815, t13817, t13818, t13820, t13822)
}

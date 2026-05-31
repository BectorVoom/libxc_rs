//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 546/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk546<F: Float>(t2790: F, t564: F, t1006: F, t612: F, t1883: F, t582: F, t996: F, t561: F, t198: F, t34: F, t2735: F, t1046: F, t633: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t2792 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t2790 * t564;
    let t2794 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t1006 * t612;
    let t2795 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t1883;
    let t2796 = t582 * t996;
    let t2797 = t561 * t2796;
    let t2798 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t2797;
    let t2799 = t198 * t34;
    let t2800 = t2735 * t2799;
    let t2802 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t561 * t2800;
    let t2806 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t633 * t1046;
    (t2792, t2794, t2795, t2796, t2797, t2798, t2799, t2800, t2802, t2806)
}

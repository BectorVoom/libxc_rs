//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 900/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk900<F: Float>(t211: F, t7844: F, t2826: F, t612: F, t1006: F, t1868: F, t1798: F, t2741: F, t219: F, t5400: F, t7283: F, t639: F) -> (F, F, F, F, F) {
    let t7845 = t211 * t7844;
    let t7846 = F::cast_from(4.0_f64) / F::cast_from(135.0_f64) * t7845;
    let t7848 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t2826 * t612;
    let t7850 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t1006 * t1868;
    let t7852 = F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t2741 * t1798;
    let t7853 = t5400 * t219;
    let t7854 = t7853 * t7283;
    let t7856 = F::cast_from(32.0_f64) / F::cast_from(81.0_f64) * t639 * t7854;
    (t7846, t7848, t7850, t7852, t7856)
}

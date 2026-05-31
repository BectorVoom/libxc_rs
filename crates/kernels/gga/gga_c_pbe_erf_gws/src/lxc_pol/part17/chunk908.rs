//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 908/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk908<F: Float>(t2602: F, t5493: F, t639: F, t1406: F, t2625: F, t1885: F, t1820: F, t2631: F, t5018: F, t587: F, t1017: F, t1804: F, t5175: F) -> (F, F, F, F) {
    let t7925 = t5493 * t2602;
    let t7927 = F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t639 * t7925;
    let t7928 = t2625 * t1406;
    let t7929 = t1885 * t7928;
    let t7931 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t1820 * t7929;
    let t7932 = t5018 * t2631;
    let t7934 = F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t587 * t7932;
    let t7936 = t5175 * t1017 * t1804;
    (t7927, t7931, t7934, t7936)
}

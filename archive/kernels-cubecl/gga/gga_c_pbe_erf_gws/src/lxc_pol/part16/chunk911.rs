//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 911/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk911<F: Float>(t202: F, t2814: F, t184: F, t619: F, t1735: F, t2741: F, t1672: F, t996: F, t561: F, t2799: F, t7776: F, t2768: F, t418: F) -> (F, F, F, F, F) {
    let t7950 = t202 * t2814;
    let t7951 = t7950 * t184;
    let t7953 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t7951 * t619;
    let t7955 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t2741 * t1735;
    let t7956 = t1672 * t996;
    let t7957 = t561 * t7956;
    let t7958 = F::cast_from(8.0_f64) / F::cast_from(135.0_f64) * t7957;
    let t7959 = t7776 * t2799;
    let t7960 = t561 * t7959;
    let t7961 = F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t7960;
    let t7962 = t2768 * t418;
    (t7953, t7955, t7958, t7961, t7962)
}

//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 350/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk350<F: Float>(t247: F, t991: F, t251: F, t108: F, t726: F, t728: F, t950: F, t954: F, t1008: F, t1012: F, t1021: F, t1026: F, t1035: F, t1039: F, t1048: F, t256: F, t267: F, t585: F, t638: F, t716: F, t722: F, t725: F, t737: F, t999: F) -> (F, F, F, F) {
    let t1061 = t991 * t247;
    let t1062 = t1061 * t251;
    let t1069 = (F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t726 * t950 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t728 * t954) * t108;
    let t1072 = t999 + t1008 + t585 + t1012 - t1021 + t1026 + t1035 + t638 + t1039 - t1048 + t1062 * t256 / F::cast_from(3.0_f64) + t716 + t722 + t725 - t1069 * t267 / F::cast_from(15.0_f64) - t737;
    (t1061, t1062, t1069, t1072)
}

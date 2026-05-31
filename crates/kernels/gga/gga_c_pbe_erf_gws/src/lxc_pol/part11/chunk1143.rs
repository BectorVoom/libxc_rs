//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1143/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1143<F: Float>(t1620: F, t1809: F, t3473: F, t3562: F, t1044: F, t12513: F, t1815: F, t639: F, t16801: F, t42094: F, t954: F, t1821: F, t47438: F, t587: F) -> (F, F, F, F) {
    let t48187 = F::cast_from(16.0_f64) / F::cast_from(15.0_f64) * t1620 * t1809 * t3473 * t3562;
    let t48191 = F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t639 * t1815 * t12513 * t1044;
    let t48195 = F::cast_from(32.0_f64) / F::cast_from(15.0_f64) * t639 * t16801 * t42094 * t954;
    let t48198 = F::cast_from(32.0_f64) / F::cast_from(45.0_f64) * t587 * t1821 * t47438;
    (t48187, t48191, t48195, t48198)
}

//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1262/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1262<F: Float>(t11630: F, t11782: F, t11668: F, t13243: F, t1105: F, t13489: F, t2147: F, t3116: F, t337: F, t3854: F, t6241: F, t11478: F, t3139: F, t8903: F) -> (F, F, F, F, F) {
    let t50049 = t11782 * t11630 / F::cast_from(16.0_f64);
    let t50051 = t11668 * t13243 / F::cast_from(6.0_f64);
    let t50056 = t3116 * t2147 * t337 * t13489 * t1105 / F::cast_from(12.0_f64);
    let t50069 = t6241 * t3854;
    let t50073 = F::cast_from(3.0_f64) / F::cast_from(8.0_f64) * t8903 * t3139 * t11478 * t50069;
    (t50049, t50051, t50056, t50069, t50073)
}

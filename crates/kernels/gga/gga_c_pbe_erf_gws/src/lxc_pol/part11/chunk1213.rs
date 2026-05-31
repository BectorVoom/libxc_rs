//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1213/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1213<F: Float>(t12138: F, t12234: F, t13096: F, t13127: F, t13174: F, t13217: F, t13227: F, t2408: F, t2409: F, t2503: F, t35481: F, t3913: F, t3921: F, t43887: F, t43889: F, t43903: F, t46930: F, t8589: F, t9820: F, t9890: F, t9899: F, t9902: F) -> F {
    let t49219 = t2408 * t2409 * t8589 * t13227 / F::cast_from(4.0_f64) + t3921 * t12234 / F::cast_from(16.0_f64) + t13127 * t2503 / F::cast_from(24.0_f64) + t13174 * t2503 / F::cast_from(24.0_f64) + F::cast_from(35.0_f64) / F::cast_from(72.0_f64) * t35481 - F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t43887 - F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t43889 + F::cast_from(7.0_f64) / F::cast_from(3.0_f64) * t43903 - t9902 * t13217 / F::cast_from(24.0_f64) - t3913 * t9899 / F::cast_from(16.0_f64) + F::cast_from(3.0_f64) / F::cast_from(8.0_f64) * t3913 * t9820 + t46930 * t13096 / F::cast_from(16.0_f64) - t3913 * t12138 / F::cast_from(4.0_f64) - t3913 * t9890 / F::cast_from(8.0_f64);
    t49219
}

//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2730/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2730<F: Float>(t10985: F, t15017: F, t39557: F, t39558: F, t39562: F, t39565: F, t39567: F, t39570: F, t39573: F, t40968: F, t40970: F, t40973: F, t40978: F, t50198: F, t50201: F, t50205: F, t50209: F) -> F {
    let t50214 = t15017 * t10985;
    let t50216 = t39557 - F::cast_from(0.13878983423218070567e-1_f64) * t39558 - F::cast_from(0.19514881078765566037e-2_f64) * t39562 + F::cast_from(0.39029762157531132075e-2_f64) * t39565 - F::cast_from(0.39029762157531132075e-1_f64) * t39567 + F::cast_from(0.16463622957338778996e-1_f64) * t39570 - F::cast_from(0.39029762157531132075e-2_f64) * t39573 + F::cast_from(0.16463622957338778996e-1_f64) * t50198 + F::cast_from(0.29272321618148349057e-1_f64) * t50201 + F::cast_from(0.16463622957338778996e-1_f64) * t40968 - F::cast_from(0.30356481678079769392e-1_f64) * t50205 + F::cast_from(0.58544643236296698113e-1_f64) * t50209 - F::cast_from(0.7805952431506226415e-2_f64) * t40970 + F::cast_from(0.54878743191129263322e-2_f64) * t40973 - F::cast_from(0.58911598146606471822e-3_f64) * t40978 - F::cast_from(0.46263278077393568556e-2_f64) * t50214;
    t50216
}

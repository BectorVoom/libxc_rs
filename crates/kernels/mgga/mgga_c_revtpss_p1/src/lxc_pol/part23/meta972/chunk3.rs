//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3294/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3294<F: Float>(t1399: F, t14122: F, t14193: F, t21981: F, t22016: F, t22858: F, t23037: F, t46526: F, t46554: F, t49167: F, t49439: F, t5659: F, t5745: F, t5755: F, t75068: F, t75071: F, t75074: F, t820: F, t85614: F, t86445: F, t86470: F) -> F {
    let t86498 = F::cast_from(0.69394917116090352834e-2_f64) * t75068 - F::cast_from(0.29272321618148349057e-1_f64) * t75071 - F::cast_from(0.19514881078765566037e-2_f64) * t75074 + F::cast_from(0.39512695097613069591e1_f64) * t5745 * t14122 * t23037 + F::cast_from(0.15805078039045227836e2_f64) * t49439 * t86445 * t85614 - F::cast_from(0.23707617058567841754e2_f64) * t14193 * t86445 * t22016 - F::cast_from(0.39512695097613069591e1_f64) * t820 * t46554 * t22858 + F::cast_from(0.11044544084478153697e-3_f64) * t46526 + F::cast_from(0.19514881078765566038e-2_f64) * t49167 - F::cast_from(0.39512695097613069592e1_f64) * t5755 * t21981 * t5659 - F::cast_from(0.19756347548806534796e1_f64) * t5755 * t86470 * t1399;
    t86498
}

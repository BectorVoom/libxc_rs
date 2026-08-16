//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3294/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3294(t1399: f64, t14122: f64, t14193: f64, t21981: f64, t22016: f64, t22858: f64, t23037: f64, t46526: f64, t46554: f64, t49167: f64, t49439: f64, t5659: f64, t5745: f64, t5755: f64, t75068: f64, t75071: f64, t75074: f64, t820: f64, t85614: f64, t86445: f64, t86470: f64) -> f64 {
    let t86498 = 0.69394917116090352834e-2_f64 * t75068 - 0.29272321618148349057e-1_f64 * t75071 - 0.19514881078765566037e-2_f64 * t75074 + 0.39512695097613069591e1_f64 * t5745 * t14122 * t23037 + 0.15805078039045227836e2_f64 * t49439 * t86445 * t85614 - 0.23707617058567841754e2_f64 * t14193 * t86445 * t22016 - 0.39512695097613069591e1_f64 * t820 * t46554 * t22858 + 0.11044544084478153697e-3_f64 * t46526 + 0.19514881078765566038e-2_f64 * t49167 - 0.39512695097613069592e1_f64 * t5755 * t21981 * t5659 - 0.19756347548806534796e1_f64 * t5755 * t86470 * t1399;
    t86498
}

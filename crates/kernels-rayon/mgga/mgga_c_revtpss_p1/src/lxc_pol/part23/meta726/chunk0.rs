//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2492/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2492(t49186: f64, t10142: f64, t14113: f64, t49180: f64, t10136: f64, t14239: f64, t4101: f64, t5740: f64, t9288: f64, t40270: f64, t5737: f64, t10073: f64, t14207: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t49187 = 0.69394917116090352834e-2_f64 * t49186;
    let t49189 = t49180 * t14113 * t10142;
    let t49190 = 0.34697458558045176417e-2_f64 * t49189;
    let t49198 = t14239 * t10136;
    let t49199 = 0.39029762157531132076e-1_f64 * t49198;
    let t49203 = t4101 * t5740 * t9288;
    let t49210 = t40270 * t5737;
    let t49238 = t10073 * t14207;
    (t49187, t49190, t49199, t49203, t49210, t49238)
}

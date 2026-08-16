//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1084/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1084(t28140: f64, t28174: f64, t28222: f64, t28248: f64, t2205: f64, t5394: f64, t11223: f64, t15109: f64, t27141: f64, t27986: f64, t27988: f64, t27989: f64, t27992: f64, t27998: f64, t28001: f64, t28008: f64, t28073: f64, t28076: f64, t3669: f64, t437: f64, t5363: f64, t7809: f64, t8108: f64) -> (f64, f64, f64) {
    let t28250 = t28140 + t28174 + t28222 + t28248;
    let t28253 = t2205 * t5394;
    let t28256 = 2.0_f64 * t11223 * t8108 - t15109 * t2205 + 2.0_f64 * t27141 * t5363 + 2.0_f64 * t28073 * t3669 + 2.0_f64 * t28076 * t3669 + t28250 * t437 + 2.0_f64 * t28253 * t3669 - t5394 * t7809 - t27986 + t27988 + t27989 + t27992 - t27998 + t28001 + t28008;
    (t28250, t28253, t28256)
}

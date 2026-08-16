//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2717/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2717(t39845: f64, t54456: f64, t39615: f64, t39642: f64, t39655: f64, t39658: f64, t39844: f64, t57203: f64, t57204: f64, t57205: f64, t57206: f64, t57207: f64, t57209: f64, t57210: f64, t57212: f64, t57213: f64, t57214: f64) -> (f64, f64, f64) {
    let t57215 = 120.0_f64 * t39845;
    let t57216 = 48.0_f64 * t54456;
    let t57217 = -t57203 - t57204 - t57205 + t39615 + t57206 + t57207 + t57209 + t57210 + t57212 + t39642 - t57213 + t57214 - t39655 - t39658 + t39844 + t57215 - t57216;
    (t57215, t57216, t57217)
}

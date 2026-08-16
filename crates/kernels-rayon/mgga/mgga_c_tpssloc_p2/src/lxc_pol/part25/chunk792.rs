//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 792/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk792(t10259: f64, t2990: f64, t2262: f64, t972: f64, t10186: f64, t10192: f64, t10196: f64, t10200: f64, t10204: f64, t10209: f64, t10219: f64, t10226: f64, t10229: f64, t10233: f64, t10238: f64, t10242: f64, t10246: f64, t10251: f64, t10256: f64, t2960: f64, t2982: f64, t2986: f64, t2991: f64, t973: f64, t980: f64) -> (f64, f64) {
    let t10260 = t10259 * t2990;
    let t10263 = t2262 * t972;
    let t10266 = 0.44444444444444444443e-2_f64 * t10186 * t2991 - 0.55555555555555555554e-3_f64 * t10192 + 0.11111111111111111111e-2_f64 * t2986 * t10196 + 0.16666666666666666666e-2_f64 * t973 * t10200 + 0.27777777777777777777e-3_f64 * t973 * t10204 - 0.24999999999999999999e-2_f64 * t973 * t10209 + 0.86419753086419753084e-3_f64 * t973 * t10219 - 0.29629629629629629629e-2_f64 * t2960 * t2982 - 0.18518518518518518518e-3_f64 * t10226 + 0.27777777777777777777e-3_f64 * t10229 + 0.37037037037037037036e-3_f64 * t10233 - 0.11111111111111111111e-2_f64 * t2986 * t10238 - 0.83333333333333333331e-3_f64 * t2986 * t10242 - 0.83333333333333333331e-3_f64 * t2986 * t10246 - 0.16666666666666666666e-2_f64 * t2986 * t10251 + 0.16666666666666666666e-2_f64 * t2986 * t10256 - 0.83333333333333333331e-3_f64 * t2986 * t10260 + 0.81481481481481481478e-2_f64 * t10263 * t980;
    (t10263, t10266)
}

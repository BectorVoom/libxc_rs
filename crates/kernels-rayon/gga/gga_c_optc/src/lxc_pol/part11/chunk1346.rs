//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1346/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1346(t1102: f64, t26214: f64, t26217: f64, t58311: f64, t15146: f64, t18183: f64, t1220: f64, t1221: f64, t4297: f64, t43571: f64, t53279: f64, t53281: f64, t53290: f64, t53293: f64, t53299: f64, t58295: f64, t58308: f64, t58310: f64, t914: f64) -> (f64, f64) {
    let t58315 = 0.91080982599109921211e5_f64 * t1102 * t26214 * t58311 * t26217;
    let t58316 = t15146 * t18183;
    let t58319 = t1220 * t914 * t1221 * t58295 / 6.0_f64 - 64.0_f64 / 27.0_f64 * t53279 - 32.0_f64 / 9.0_f64 * t53281 - 400.0_f64 / 81.0_f64 * t53290 - 4.0_f64 / 3.0_f64 * t53293 - 200.0_f64 / 9.0_f64 * t53299 + 200.0_f64 / 9.0_f64 * t43571 + t58308 - t58310 - t58315 + 800.0_f64 / 81.0_f64 * t4297 * t58316;
    (t58315, t58319)
}

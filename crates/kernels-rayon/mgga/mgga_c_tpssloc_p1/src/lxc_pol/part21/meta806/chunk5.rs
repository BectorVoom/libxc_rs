//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2803/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2803(t59134: f64, t59178: f64, t59197: f64, t59227: f64, t225: f64, t13222: f64, t13223: f64, t13228: f64, t16912: f64, t16969: f64, t210: f64, t237: f64, t2379: f64, t249: f64, t2643: f64, t41130: f64, t41134: f64, t41139: f64, t41161: f64, t41341: f64, t41363: f64, t41365: f64, t41373: f64, t41386: f64, t4178: f64, t46692: f64, t47017: f64, t47093: f64, t47230: f64, t47267: f64, t5567: f64, t5571: f64, t59100: f64, t9559: f64, t9642: f64) -> (f64, f64, f64) {
    let t59229 = t59134 + t59178 + t59197 + t59227;
    let t59230 = t59229 * t225;
    let t59235 = -595.0_f64 / 5184.0_f64 * t41130 + 119.0_f64 / 13824.0_f64 * t41134 + t41139 - 119.0_f64 / 6912.0_f64 * t41341 - 119.0_f64 / 3456.0_f64 * t47093 + t9642 * t16969 / 192.0_f64 + 595.0_f64 / 5184.0_f64 * t41363 - 119.0_f64 / 13824.0_f64 * t41365 - 119.0_f64 / 13824.0_f64 * t41373 + 119.0_f64 / 6912.0_f64 * t41386 - 35.0_f64 / 54.0_f64 * t47230 + 7.0_f64 / 6.0_f64 * t59100 + 5.0_f64 / 4.0_f64 * t41161 * t210 * t5567 * t2379 - t9559 * t210 * t5571 * t2379 / 4.0_f64 + t2643 * t13222 * t13223 * t16912 / 192.0_f64 + t4178 * t46692 * t13228 * t47017 / 128.0_f64 + t59230 * t237 * t249 / 3072.0_f64 - 35.0_f64 / 576.0_f64 * t47267;
    (t59229, t59230, t59235)
}

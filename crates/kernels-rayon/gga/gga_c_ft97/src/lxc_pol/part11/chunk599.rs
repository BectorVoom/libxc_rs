//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 599/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk599(t2: f64, t8326: f64, t7794: f64, t1775: f64, t1788: f64, t1793: f64, t462: f64, t8301: f64, t8302: f64, t8305: f64, t8308: f64, t8311: f64, t8316: f64, t8319: f64, t8322: f64, t8324: f64, t92: f64) -> (f64, f64, f64) {
    let t8327 = t8326 * t2;
    let t8328 = t8327 * t7794;
    let t8331 = t1775 * t1788;
    let t8333 = t1775 * t1793;
    let t8335 = -t8301 - 4.0_f64 / 3.0_f64 * t8302 - t92 * t8305 - 2.0_f64 * t462 * t8308 + 2.0_f64 * t462 * t8311 + 4.0_f64 / 3.0_f64 * t462 * t8316 - 2.0_f64 / 3.0_f64 * t462 * t8319 + t462 * t8322 + t462 * t8324 + 2.0_f64 / 3.0_f64 * t462 * t8328 - 2.0_f64 / 3.0_f64 * t8331 - 2.0_f64 / 3.0_f64 * t8333;
    (t8327, t8328, t8335)
}

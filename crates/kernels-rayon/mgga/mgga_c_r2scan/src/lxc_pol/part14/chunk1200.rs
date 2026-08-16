//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1200/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1200(t12365: f64, t41280: f64, t41283: f64, t41285: f64, t41286: f64, t41289: f64, t41291: f64, t41294: f64, t41296: f64, t41300: f64, t41305: f64, t41308: f64, t41311: f64, t41314: f64, t41316: f64, t41319: f64, t41322: f64, t885: f64) -> f64 {
    let t41323 = 2.0_f64 * t12365 * t885 + t41280 - t41283 + t41285 + t41286 - t41289 - t41291 + t41294 + t41296 - t41300 - t41305 - t41308 + t41311 - t41314 - t41316 + t41319 + t41322;
    t41323
}

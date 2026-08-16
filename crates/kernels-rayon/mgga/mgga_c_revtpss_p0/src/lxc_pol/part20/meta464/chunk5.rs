//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1769/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1769(t213: f64, t47343: f64, t47348: f64, t47351: f64, t47352: f64, t47354: f64, t47359: f64, t47364: f64, t47369: f64, t47375: f64, t47379: f64, t47381: f64, t546: f64) -> f64 {
    let t47383 = 0.65854491829355115987e0_f64 * t213 * t546 * t47343 + 0.78548797528808629095e-3_f64 * t47348 - t47351 + 0.1040793657534163522e-1_f64 * t47352 - 0.11708928647259339623e0_f64 * t47354 - 0.39029762157531132076e-1_f64 * t47359 - 0.69394917116090352835e-2_f64 * t47364 - 0.39029762157531132076e-1_f64 * t47369 - 0.23417857294518679245e0_f64 * t47375 + 0.23417857294518679245e0_f64 * t47379 - 0.44178176337912614788e-3_f64 * t47381;
    t47383
}

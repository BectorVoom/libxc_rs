//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1486/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1486(t6880: f64, t9779: f64, t22062: f64, t9775: f64, t22068: f64, t9765: f64, t22022: f64, t22061: f64, t808: f64, t9845: f64, t22182: f64, t47215: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t74279 = t9779 * t6880;
    let t74281 = t9775 * t22062;
    let t74290 = t9765 * t22068;
    let t74299 = t9775 * t22022;
    let t74304 = t9845 * t808 * t22061;
    let t74322 = t47215 * t22182;
    (t74279, t74281, t74290, t74299, t74304, t74322)
}

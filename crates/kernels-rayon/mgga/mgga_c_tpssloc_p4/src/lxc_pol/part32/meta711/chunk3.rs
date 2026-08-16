//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2228/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2228(t28334: f64, t6547: f64, t28322: f64, t6579: f64, t16762: f64, t1888: f64, t6646: f64, t16828: f64, t1484: f64, t1519: f64, t25038: f64, t25248: f64, t776: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t98374 = t6547 * t28334;
    let t98380 = t6579 * t28322;
    let t98384 = t1888 * t6646 * t16762;
    let t98387 = t1888 * t6646 * t16828;
    let t98389 = t1519 * t1484;
    let t98392 = t25038 * t25248 * t98389 * t776;
    (t98374, t98380, t98384, t98387, t98389, t98392)
}

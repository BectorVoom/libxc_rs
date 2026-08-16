//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2635/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2635(t13774: f64, t2661: f64, t5675: f64, t9934: f64, t1868: f64, t4056: f64, t1882: f64, t2682: f64, t4000: f64, t5677: f64, t820: f64, t13985: f64, t46740: f64) -> (f64, f64, f64, f64, f64) {
    let t48462 = t2661 * t9934 * t13774 * t5675;
    let t48466 = t1868 * t4056;
    let t48475 = t1882 * t4056;
    let t48486 = t820 * t4000 * t2682 * t5677;
    let t48487 = 0.34013387707001991332e-1_f64 * t48486;
    let t48488 = t46740 * t13985;
    (t48462, t48466, t48475, t48487, t48488)
}

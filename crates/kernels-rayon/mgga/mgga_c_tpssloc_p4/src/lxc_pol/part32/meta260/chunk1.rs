//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1171/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1171(t1352: f64, t6987: f64, t553: f64, t6955: f64, t1332: f64, t1336: f64, t2013: f64, t544: f64, t6967: f64, t6971: f64, t6975: f64, t6980: f64, t6984: f64) -> (f64, f64, f64) {
    let t6988 = t6987 * t1352;
    let t6990 = t553 * t6955;
    let t6992 = -t6967 - 0.16449340668482264365e-1_f64 * t6971 - t6975 - 0.82246703342411321825e-2_f64 * t6980 + 0.82246703342411321825e-2_f64 * t6984 + t1332 * t2013 - t1336 * t6988 + t544 * t6990;
    (t6988, t6990, t6992)
}

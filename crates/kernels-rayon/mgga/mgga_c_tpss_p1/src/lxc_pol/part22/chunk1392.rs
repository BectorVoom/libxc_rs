//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1392/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1392(t1665: f64, t5960: f64, t1275: f64, t6458: f64, t1673: f64, t5941: f64, t20697: f64, t546: f64, t1856: f64, t4543: f64, t1278: f64, t1284: f64, t20649: f64, t3: f64, t3399: f64, t550: f64, t62171: f64, t63116: f64, t63167: f64, t63169: f64, t67795: f64, t67800: f64, t67843: f64) -> f64 {
    let t67849 = 2.0_f64 * t1665 * t5960;
    let t67851 = 2.0_f64 * t1275 * t6458;
    let t67853 = 2.0_f64 * t5941 * t1673;
    let t67858 = 2.0_f64 * t546 * t20697;
    let t67860 = 2.0_f64 * t4543 * t1856;
    let t67861 = t1278 * (t67800 + t67843) + t62171 + t63169 + 2.0_f64 * t20649 * t1284 + t67849 + t67851 + t63116 + t67853 + t3 * t67795 * t550 + t63167 + t3399 * t6458 + t67858 + t67860;
    t67861
}

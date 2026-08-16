//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1299/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1299(t1275: f64, t6458: f64, t1673: f64, t5941: f64, t20697: f64, t546: f64, t1856: f64, t4543: f64, t1848: f64, t4562: f64, t20648: f64, t550: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t67851 = 2.0_f64 * t1275 * t6458;
    let t67853 = 2.0_f64 * t5941 * t1673;
    let t67858 = 2.0_f64 * t546 * t20697;
    let t67860 = 2.0_f64 * t4543 * t1856;
    let t67868 = 2.0_f64 * t1848 * t4562;
    let t67874 = 2.0_f64 * t20648 * t550;
    (t67851, t67853, t67858, t67860, t67868, t67874)
}

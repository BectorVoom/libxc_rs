//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1024/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1024(t1568: f64, t7063: f64, t33: f64, t41154: f64, t116: f64, t29421: f64, t1203: f64, t471: f64, t11239: f64, t1811: f64, t1828: f64, t1774: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t98848 = t7063 * t1568;
    let t100981 = t41154 * t33;
    let t104115 = t29421 * t116;
    let t104504 = t471 * t1203;
    let t104527 = t1811 * t11239;
    let t105236 = t1828 * t1203;
    let t105270 = t1774 * t1203;
    (t98848, t100981, t104115, t104504, t104527, t105236, t105270)
}

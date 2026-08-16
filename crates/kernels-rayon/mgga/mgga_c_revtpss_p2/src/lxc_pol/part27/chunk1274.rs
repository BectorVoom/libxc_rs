//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1274/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1274(t26054: f64, t9671: f64, t1419: f64, t7063: f64, t25898: f64, t25901: f64, t136: f64, t2457: f64, t7307: f64, t25944: f64, t26035: f64, t686: f64, t72: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t94799 = t26054 * t9671;
    let t94801 = t7063 * t1419;
    let t94802 = t94801 * t25898;
    let t94803 = t94802 * t25901;
    let t94806 = t7307 * t136 * t2457;
    let t94807 = t25944 * t94806;
    let t94810 = t26035 * t72 * t686;
    (t94799, t94801, t94803, t94806, t94807, t94810)
}

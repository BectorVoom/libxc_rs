//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1022/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1022(t10308: f64, t1466: f64, t7063: f64, t860: f64, t1444: f64, t543: f64, t1419: f64, t11239: f64, t1269: f64, t42859: f64, t487: f64, t1294: f64, t471: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t60224 = t1466 * t10308;
    let t93341 = t7063 * t860;
    let t94396 = t543 * t1444;
    let t94801 = t7063 * t1419;
    let t96881 = t1269 * t11239;
    let t96886 = t487 * t42859;
    let t96928 = t471 * t1294;
    (t60224, t93341, t94396, t94801, t96881, t96886, t96928)
}

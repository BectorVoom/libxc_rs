//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1028/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1028(t11365: f64, t898: f64, t11143: f64, t11231: f64, t11236: f64, t11292: f64, t11295: f64, t11318: f64, t11355: f64, t11357: f64, t11359: f64, t11361: f64, t11363: f64) -> (f64, f64) {
    let t11367 = 0.35089341735807877242e1_f64 * t898 * t11365;
    let t11368 = -t11355 - t11357 - t11359 - t11361 + t11363 + t11318 - t11292 + t11295 - t11231 + t11236 - t11367 + t11143;
    (t11367, t11368)
}

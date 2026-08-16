//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 322/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk322(t1361: f64, t240: f64, t1336: f64, t531: f64, t556: f64, t241: f64, t67: f64, t1307: f64, t820: f64) -> (f64, f64, f64, f64, f64) {
    let t1362 = t1361 * t240;
    let t1363 = t1336 * t1362;
    let t1365 = 1.0_f64 / t556 / t531;
    let t1367 = t241 * t1365 * t67;
    let t1369 = t1367 * t820 * t1307;
    (t1362, t1363, t1365, t1367, t1369)
}

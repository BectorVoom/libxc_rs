//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1089/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1089(t26895: f64, t26982: f64, t27183: f64, t27238: f64, t3: f64, t112: f64, t7945: f64, t1458: f64, t7056: f64, t2039: f64, t4072: f64, t671: f64, t7801: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t27240 = t26895 + t26982 + t27183 + t27238;
    let t27241 = t3 * t27240;
    let t27254 = t7945 * t112;
    let t27273 = t7056 * t1458;
    let t27276 = t2039 * t4072;
    let t27281 = t7801 * t671;
    (t27240, t27241, t27254, t27273, t27276, t27281)
}

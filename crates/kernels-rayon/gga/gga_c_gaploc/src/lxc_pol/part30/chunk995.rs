//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 995/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk995(t10802: f64, t5559: f64, t1052: f64, t2728: f64, t1960: f64, t10284: f64, t10287: f64, t10291: f64, t10292: f64, t10294: f64, t10303: f64, t10304: f64, t10307: f64, t10795: f64, t10797: f64, t10798: f64, t10800: f64, t1955: f64, t331: f64, t3511: f64, t841: f64) -> (f64, f64) {
    let t10804 = 6.0_f64 * t5559 * t10802;
    let t10805 = t1052 * t2728;
    let t10807 = 2.0_f64 * t1960 * t10805;
    let t10808 = t10795 * t331 - t10800 * t841 - t1955 * t3511 + t10284 - t10287 - t10291 + t10292 - t10294 - t10303 + t10304 - t10307 - t10797 - t10798 - t10804 + t10807;
    (t10805, t10808)
}

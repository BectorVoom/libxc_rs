//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1433/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1433(t1824: f64, t8617: f64, t1338: f64, t33266: f64, t1985: f64, t1998: f64, t214: f64, t27051: f64, t1992: f64, t550: f64, t6976: f64, t93505: f64) -> (f64, f64, f64, f64) {
    let t122471 = t8617 * t1824;
    let t122475 = t1338 * t33266;
    let t122483 = t1985 * t214 * t1998 * t27051;
    let t122488 = t1992 * t6976 * t93505 * t550;
    (t122471, t122475, t122483, t122488)
}

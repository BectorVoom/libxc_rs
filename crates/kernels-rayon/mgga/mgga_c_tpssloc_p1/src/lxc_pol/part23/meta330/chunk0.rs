//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1098/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1098(t1441: f64, t1458: f64, t1799: f64, t1824: f64, t1484: f64, t1530: f64, t1409: f64, t1615: f64, t1845: f64, t5456: f64, t576: f64, t460: f64, t6144: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t28002 = t1441 * t1458;
    let t28099 = t1799 * t1824;
    let t28248 = t1484 * t1530;
    let t28651 = t1409 * t1615;
    let t28830 = t1799 * t1845;
    let t28893 = t576 * t5456;
    let t29614 = t6144 * t460;
    (t28002, t28099, t28248, t28651, t28830, t28893, t29614)
}

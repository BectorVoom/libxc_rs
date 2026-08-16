//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 748/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk748(t1557: f64, t469: f64, t3188: f64, t432: f64, t26: f64, t356: f64, t1570: f64, t1800: f64, t942: f64, t1565: f64, t11718: f64, t11720: f64, t11724: f64, t11728: f64, t11732: f64, t11734: f64, t11735: f64, t11738: f64, t11741: f64, t11745: f64, t11746: f64, t11749: f64, t11755: f64, t3139: f64, t462: f64, t8289: f64, t8298: f64, t8301: f64, t8302: f64, t8331: f64) -> (f64, f64) {
    let t11756 = t469 * t1557;
    let t11757 = t3188 * t432;
    let t11758 = t11756 * t11757;
    let t11761 = t26 * t356;
    let t11762 = t469 * t1570;
    let t11763 = t11762 * t11757;
    let t11766 = t1800 * t942;
    let t11767 = t11766 * t1565;
    let t11771 = 22.0_f64 / 9.0_f64 * t11718 - 4.0_f64 / 27.0_f64 * t11720 + 4.0_f64 * t462 * t11724 - 6.0_f64 * t462 * t11728 - t11732 - t8301 - t11734 + 2.0_f64 / 3.0_f64 * t462 * t11735 + 8.0_f64 / 3.0_f64 * t3139 * t11738 + 4.0_f64 / 3.0_f64 * t3139 * t11741 - t11745 - 2.0_f64 * t462 * t11746 - 2.0_f64 / 3.0_f64 * t462 * t11749 - 2.0_f64 / 3.0_f64 * t8289 + t8298 / 3.0_f64 - 8.0_f64 / 9.0_f64 * t8302 + 4.0_f64 / 9.0_f64 * t11755 * t11758 - 4.0_f64 / 3.0_f64 * t11761 * t11763 - 4.0_f64 / 3.0_f64 * t11761 * t11767 - 2.0_f64 / 9.0_f64 * t8331;
    (t11761, t11771)
}

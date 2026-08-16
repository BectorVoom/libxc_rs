//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1712/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1712(t1530: f64, t1649: f64, t28: f64, t5660: f64, t191: f64, t192: f64, t6295: f64, t1390: f64, t6330: f64, t1799: f64, t1845: f64, t6347: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t28792 = t1649 * t1530;
    let t28795 = t28 * t5660;
    let t28821 = t6295 * t191 * t192;
    let t28826 = t1390 * t6330;
    let t28830 = t1799 * t1845;
    let t28834 = t1390 * t6347;
    (t28792, t28795, t28821, t28826, t28830, t28834)
}

//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1088/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1088(t1882: f64, t34644: f64, t34703: f64, t34511: f64, t487: f64, t103510: f64, t110: f64, t11490: f64, t11593: f64, t11902: f64, t1339: f64, t137797: f64, t138057: f64, t144991: f64, t1901: f64, t1902: f64, t25595: f64, t25846: f64, t26166: f64, t26372: f64, t26436: f64, t3052: f64, t3214: f64, t3219: f64, t32597: f64, t32598: f64, t34379: f64, t446: f64, t452: f64, t47659: f64, t499: f64, t5617: f64, t6564: f64, t7281: f64, t8411: f64, t8417: f64) -> (f64, f64, f64, f64) {
    let t146680 = t1882 * t34644;
    let t146682 = t1882 * t34703;
    let t146693 = t487 * t34511;
    let t146741 = t1901 * t11902 * t32598 / 9.0_f64 + 2.0_f64 / 9.0_f64 * t11593 * t1902 * t32597 * t3052 - 2.0_f64 / 3.0_f64 * t1901 * t11490 * t137797 * t3214 - 2.0_f64 * t1901 * t26372 * t8417 * t7281 * t3219 - 4.0_f64 / 3.0_f64 * t1901 * t11490 * t26166 * t25595 - 2.0_f64 * t446 * t8411 * t499 * t34379 - 2.0_f64 * t446 * t8411 * t110 * t144991 - 2.0_f64 / 3.0_f64 * t446 * t452 * t6564 * t5617 - 2.0_f64 / 3.0_f64 * t446 * t452 * t1339 * t25846 - t138057 + 4.0_f64 / 9.0_f64 * t47659 * t103510 * t26436;
    (t146680, t146682, t146693, t146741)
}

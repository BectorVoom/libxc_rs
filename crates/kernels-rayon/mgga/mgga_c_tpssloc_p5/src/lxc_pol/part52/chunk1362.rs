//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1362/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1362(t1873: f64, t26135: f64, t3941: f64, t4072: f64, t576: f64, t8319: f64, t7769: f64, t96351: f64, t23880: f64, t26542: f64, t26545: f64, t75795: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t120830 = 54.0_f64 * t3941 * t1873 * t26135;
    let t120833 = t576 * t4072;
    let t120835 = 27.0_f64 * t120833 * t8319;
    let t120836 = t96351 * t7769;
    let t120838 = t23880 * t26542;
    let t120840 = t23880 * t26545;
    let t120848 = 27.0_f64 * t75795 * t8319;
    (t120830, t120835, t120836, t120838, t120840, t120848)
}

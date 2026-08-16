//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2811/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2811(t10076: f64, t13385: f64, t13390: f64, t13401: f64, t13404: f64, t13429: f64, t16753: f64, t16759: f64, t16811: f64, t16815: f64, t17027: f64, t17034: f64, t2617: f64, t2633: f64, t2684: f64, t2732: f64, t2740: f64, t4166: f64, t4182: f64, t4281: f64, t4291: f64, t5575: f64, t5617: f64, t58226: f64, t58262: f64, t59331: f64, t812: f64, t829: f64) -> f64 {
    let t59412 = -t10076 * t5617 * t812 - 2.0_f64 * t16753 * t2732 * t812 + 14.0_f64 * t16815 * t2633 * t4281 - t17027 * t2684 * t812 + 8.0_f64 * t4182 * t4281 * t58226 + 4.0_f64 * t4182 * t4281 * t59331 - 2.0_f64 * t4291 * t58262 * t829 + 8.0_f64 * t13385 * t17034 - 4.0_f64 * t13390 * t16759 + 12.0_f64 * t13401 * t17034 + 4.0_f64 * t13404 * t17034 - 2.0_f64 * t13429 * t4166 + 4.0_f64 * t16811 * t2617 + t2740 * t5575;
    t59412
}

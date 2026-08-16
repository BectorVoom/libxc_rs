//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1083/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1083(t1799: f64, t1998: f64, t59: f64, t6926: f64, t1825: f64, t6943: f64, t6936: f64, t1814: f64, t8465: f64, t8467: f64, t5248: f64, t5249: f64, t550: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t32711 = t1998 * t59 * t1799;
    let t32712 = t6926 * t32711;
    let t32714 = t6943 * t1825;
    let t32715 = t6936 * t32714;
    let t32717 = t1814 * t8465;
    let t32718 = t32717 * t8467;
    let t32721 = t5248 * t5249 * t550;
    (t32711, t32712, t32714, t32715, t32717, t32718, t32721)
}

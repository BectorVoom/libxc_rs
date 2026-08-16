//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1120/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1120(t32785: f64, t33163: f64, t3: f64, t1873: f64, t26523: f64, t23880: f64, t7769: f64, t7010: f64, t7467: f64, t16524: f64, t8319: f64, t1458: f64, t576: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t33164 = t32785 + t33163;
    let t33165 = t3 * t33164;
    let t33177 = t26523 * t1873;
    let t33179 = t23880 * t7769;
    let t33181 = t7010 * t7467;
    let t33184 = 27.0_f64 * t16524 * t8319;
    let t33185 = t576 * t1458;
    (t33164, t33165, t33177, t33179, t33181, t33184, t33185)
}

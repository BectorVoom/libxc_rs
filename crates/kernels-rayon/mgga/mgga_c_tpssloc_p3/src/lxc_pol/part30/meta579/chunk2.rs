//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1957/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1957(t28816: f64, t28867: f64, t3: f64, t1873: f64, t20162: f64, t16524: f64, t7769: f64, t5371: f64, t7467: f64, t5456: f64, t576: f64, t1458: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t28868 = t28816 + t28867;
    let t28869 = t3 * t28868;
    let t28888 = 0.135e2_f64 * t20162 * t1873;
    let t28890 = 54.0_f64 * t16524 * t7769;
    let t28892 = 27.0_f64 * t5371 * t7467;
    let t28893 = t576 * t5456;
    let t28895 = 27.0_f64 * t28893 * t1873;
    let t28896 = t7467 * t1458;
    (t28868, t28869, t28888, t28890, t28892, t28893, t28895, t28896)
}

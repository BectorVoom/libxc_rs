//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 868/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk868(t33266: f64, t539: f64, t2016: f64, t27068: f64, t31106: f64, t31113: f64, t31115: f64, t31596: f64, t32700: f64, t32707: f64, t32733: f64, t32737: f64, t33259: f64, t568: f64) -> (f64, f64) {
    let t33267 = t539 * t33266;
    let t33269 = -t2016 * t27068 + t33259 * t568 + t33267 * t568 - t31106 - t31113 + t31115 + t31596 - t32700 + t32707 - t32733 - t32737;
    (t33267, t33269)
}

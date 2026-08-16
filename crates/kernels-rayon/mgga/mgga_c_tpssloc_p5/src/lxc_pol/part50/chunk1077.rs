//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1077/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1077(t31061: f64, t31252: f64, t3: f64, t112: f64, t8496: f64, t1873: f64, t23877: f64, t23880: f64, t7015: f64, t6534: f64, t7010: f64, t12524: f64, t8319: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t31253 = t31061 + t31252;
    let t31254 = t3 * t31253;
    let t31267 = t8496 * t112;
    let t31270 = t23877 * t1873;
    let t31272 = t23880 * t7015;
    let t31274 = t7010 * t6534;
    let t31277 = 27.0_f64 * t12524 * t8319;
    (t31253, t31254, t31267, t31270, t31272, t31274, t31277)
}

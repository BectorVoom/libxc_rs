//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2285/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2285(t55388: f64, t7015: f64, t20173: f64, t28896: f64, t28893: f64, t6534: f64, t1401: f64, t96729: f64, t16524: f64, t26542: f64, t1458: f64, t26135: f64, t3941: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t100875 = 27.0_f64 * t55388 * t7015;
    let t100879 = 54.0_f64 * t20173 * t28896;
    let t100883 = 27.0_f64 * t28893 * t6534;
    let t100885 = 0.135e2_f64 * t1401 * t96729;
    let t100887 = 54.0_f64 * t16524 * t26542;
    let t100890 = 54.0_f64 * t3941 * t26135 * t1458;
    (t100875, t100879, t100883, t100885, t100887, t100890)
}

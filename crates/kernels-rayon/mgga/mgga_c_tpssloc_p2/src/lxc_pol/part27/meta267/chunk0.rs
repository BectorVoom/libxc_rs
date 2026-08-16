//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1280/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1280(t1603: f64, t1945: f64, t1409: f64, t3: f64, t1933: f64, t1597: f64, t343: f64) -> (f64, f64, f64, f64) {
    let t7569 = t1603 * t1945;
    let t7573 = t3 * t1409;
    let t7574 = t1933 * t7573;
    let t7577 = t1597 * t343;
    (t7569, t7573, t7574, t7577)
}

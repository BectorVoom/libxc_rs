//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1284/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1284(t7285: f64, t8002: f64, t1716: f64, t2123: f64, t1751: f64, t225: f64, t497: f64) -> (f64, f64, f64, f64) {
    let t8003 = t7285 * t8002;
    let t8006 = t1716 * t2123;
    let t8009 = t1751 * t225;
    let t8010 = t8009 * t497;
    (t8003, t8006, t8009, t8010)
}

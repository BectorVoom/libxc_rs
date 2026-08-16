//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 982/1255 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk982(t12652: f64, t65: f64, t3961: f64, t628: f64, t12606: f64, t31: f64, t3967: f64, t1409: f64, t2244: f64, t9287: f64, t2267: f64, t3966: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12653 = t12652 * t65;
    let t12656 = t3961 * t628;
    let t12661 = t31 * t12606;
    let t12662 = t12661 * t65;
    let t12665 = t3967 * t628;
    let t12677 = t9287 * t1409 * t2244;
    let t12680 = t2267 * t3966;
    (t12653, t12656, t12662, t12665, t12677, t12680)
}

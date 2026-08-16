//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 705/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk705(t240: f64, t6943: f64, t1336: f64, t1354: f64, t1358: f64, t2003: f64, t552: f64, t59: f64) -> (f64, f64, f64, f64, f64) {
    let t6944 = t6943 * t240;
    let t6945 = t1336 * t6944;
    let t6946 = t6945 * t1354;
    let t6948 = t2003 * t1358;
    let t6949 = 7.0_f64 / 2304.0_f64 * t6948;
    let t6950 = t552 * t59;
    (t6944, t6945, t6946, t6949, t6950)
}

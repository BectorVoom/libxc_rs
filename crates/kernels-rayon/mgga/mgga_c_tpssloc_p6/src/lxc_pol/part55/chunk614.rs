//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 614/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk614(t1190: f64, t1751: f64, t1090: f64, t1735: f64, t3578: f64, t1216: f64, t1653: f64, t1222: f64, t1731: f64, t1744: f64, t1202: f64, t1743: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4947 = t1190 * t1751;
    let t4949 = t1735 * t1090;
    let t4950 = t3578 * t4949;
    let t4953 = t1653 * t1216;
    let t4954 = t3578 * t4953;
    let t4957 = t1731 * t1222;
    let t4959 = t1744 * t1222;
    let t4961 = t1202 * t1743;
    (t4947, t4950, t4954, t4957, t4959, t4961)
}

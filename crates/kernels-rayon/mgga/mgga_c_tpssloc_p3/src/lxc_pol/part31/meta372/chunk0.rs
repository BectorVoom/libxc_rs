//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1315/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1315(t16673: f64, t816: f64, t13278: f64, t1512: f64, t5587: f64, t9667: f64, t1510: f64, t4255: f64, t13350: f64, t120: f64, t5611: f64, t4180: f64, t4182: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t16872 = t16673 * t816;
    let t16877 = t13278 * t1512;
    let t16879 = t9667 * t5587;
    let t16887 = t1510 * t4255;
    let t16888 = t13350 * t16887;
    let t16891 = t120 * t5611;
    let t16893 = t4180 * t16891 * t4182;
    (t16872, t16877, t16879, t16887, t16888, t16891, t16893)
}

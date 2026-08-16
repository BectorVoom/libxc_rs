//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1958/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1958(t45844: f64, t7025: f64, t12571: f64, t23966: f64, t6492: f64, t7432: f64, t84195: f64, t1860: f64, t2031: f64, t2032: f64, t22527: f64, t22546: f64, t23975: f64, t26063: f64, t26067: f64, t26911: f64, t26945: f64, t6486: f64, t7026: f64, t84209: f64, t90202: f64, t90227: f64, t90232: f64, t90257: f64) -> f64 {
    let t91954 = t45844 * t7025;
    let t91957 = t12571 * t23966;
    let t91959 = 80.0_f64 / 9.0_f64 * t91957 * t6492;
    let t91961 = 80.0_f64 / 9.0_f64 * t84195 * t7432;
    let t91966 = 2.0_f64 / 3.0_f64 * t6486 * t26945 + t1860 * t2031 * t90257 / 3.0_f64 - 5.0_f64 / 3.0_f64 * t84209 * t7432 - 10.0_f64 / 3.0_f64 * t23975 * t26063 - 10.0_f64 / 3.0_f64 * t23975 * t26067 - 5.0_f64 / 3.0_f64 * t7026 * t90227 - 10.0_f64 / 3.0_f64 * t7026 * t90232 + 10.0_f64 * t91954 * t22546 + t91959 + t91961 - 4.0_f64 / 3.0_f64 * t90202 * t2032 - 10.0_f64 / 3.0_f64 * t26911 * t22527;
    t91966
}

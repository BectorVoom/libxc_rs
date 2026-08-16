//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 908/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk908(t30080: f64, t9948: f64, t3928: f64, t6449: f64, t645: f64, t6434: f64, t5016: f64, t9951: f64, t9128: f64, t1550: f64, t2060: f64, t30344: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t45209 = t30080 * t9948;
    let t45212 = t3928 * t645 * t6449;
    let t45215 = t3928 * t645 * t6434;
    let t45217 = t5016 * t9951;
    let t45219 = t9128 * t9951;
    let t45222 = t1550 * t2060 * t30344;
    (t45209, t45212, t45215, t45217, t45219, t45222)
}

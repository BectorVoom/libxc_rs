//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1107/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1107(t1061: f64, t4143: f64, t1531: f64, t2949: f64, t2931: f64, t4146: f64, t2957: f64, t4142: f64, t1530: f64, t9467: f64, t1080: f64, t4181: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t12177 = t4143 * t1061;
    let t12180 = t1531 * t2949;
    let t12183 = t4146 * t2931;
    let t12186 = t4142 * t2957;
    let t12187 = t12186 * t1061;
    let t12190 = t4146 * t2949;
    let t12193 = t1530 * t9467;
    let t12194 = t12193 * t2931;
    let t12201 = t4181 * t1080;
    (t12177, t12180, t12183, t12187, t12190, t12194, t12201)
}

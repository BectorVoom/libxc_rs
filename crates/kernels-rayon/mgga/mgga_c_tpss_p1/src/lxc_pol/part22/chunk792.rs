//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 792/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk792(t1630: f64, t3240: f64, t1206: f64, t1629: f64, t762: f64, t124: f64, t4397: f64, t236: f64, t3256: f64, t339: f64, t527: f64, t72: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4402 = t3240 * t1630;
    let t4405 = t762 * t1629 * t1206;
    let t4408 = t124 * t4397;
    let t4409 = t762 * t4408;
    let t4413 = t339 * t3256 * t236;
    let t4414 = t527 * t72;
    (t4402, t4405, t4408, t4409, t4413, t4414)
}

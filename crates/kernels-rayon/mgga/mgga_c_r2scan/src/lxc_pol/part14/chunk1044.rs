//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1044/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1044(t1234: f64, t2562: f64, t1543: f64, t5119: f64, t528: f64, t2182: f64, t979: f64, t146: f64, t5094: f64, t978: f64, t2438: f64, t2441: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t27177 = t2562 * t1234;
    let t27182 = t2562 * t1543;
    let t29418 = t5119 * t528;
    let t30370 = t2182 * t979;
    let t30792 = t146 * t5094 * t978;
    let t31912 = t2438 * t2441;
    (t27177, t27182, t29418, t30370, t30792, t31912)
}

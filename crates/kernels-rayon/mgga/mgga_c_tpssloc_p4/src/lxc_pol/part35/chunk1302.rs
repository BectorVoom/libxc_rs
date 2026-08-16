//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1302/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1302(t1864: f64, t5389: f64, t12571: f64, t1410: f64, t1437: f64, t7445: f64, t5445: f64, t2240: f64, t5399: f64, t5464: f64, t81442: f64, t22470: f64, t5488: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t96425 = t1864 * t5389;
    let t96443 = t12571 * t1410;
    let t96461 = t7445 * t1437;
    let t96469 = t1864 * t5445;
    let t96473 = t2240 * t5399;
    let t96713 = t81442 * t5464;
    let t96721 = t22470 * t5488;
    (t96425, t96443, t96461, t96469, t96473, t96713, t96721)
}

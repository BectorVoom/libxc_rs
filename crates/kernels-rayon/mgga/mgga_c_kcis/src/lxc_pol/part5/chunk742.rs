//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 742/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk742(t1489: f64, t5632: f64, t1468: f64, t1464: f64, t1307: f64, t2046: f64, t4170: f64, t4160: f64, t1650: f64, t4163: f64, t4162: f64, t1497: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5633 = t5632 * t1489;
    let t5634 = t1468 * t5633;
    let t5635 = t1464 * t5634;
    let t5637 = t2046 * t1307;
    let t5638 = t4170 * t5637;
    let t5639 = t4160 * t5638;
    let t5643 = t1650 * t1489;
    let t5644 = t4163 * t5643;
    let t5645 = t4162 * t5644;
    let t5646 = t4160 * t5645;
    let t5648 = t1650 * t1497;
    (t5633, t5634, t5635, t5637, t5638, t5639, t5644, t5645, t5646, t5648)
}

//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 858/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk858(t3754: f64, t556: f64, t5654: f64, t4170: f64, t5661: f64, t1307: f64, t2038: f64, t4162: f64, t4160: f64, t1489: f64, t2011: f64, t1495: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t5662 = t556 * t3754;
    let t5663 = t5662 * t5654;
    let t5664 = t4170 * t5663;
    let t5665 = t5661 * t5664;
    let t5667 = t2038 * t1307;
    let t5668 = t4162 * t5667;
    let t5669 = t4160 * t5668;
    let t5671 = t2011 * t1489;
    let t5672 = t1495 * t5671;
    (t5662, t5664, t5665, t5668, t5669, t5671, t5672)
}

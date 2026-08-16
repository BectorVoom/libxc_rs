//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1073/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1073(t102: f64, t4880: f64, t4859: f64, t23: f64, t821: f64, t6: f64, t107: f64, t4866: f64, t2621: f64, t9: f64, t7: f64, t118: f64) -> (f64, f64, f64, f64, f64) {
    let t13577 = t102 * t4880;
    let t13578 = t13577 * t4859;
    let t13581 = 1.0_f64 / t23 / t821;
    let t13582 = t6 * t13581;
    let t13583 = t107 * t13582;
    let t13584 = t13583 * t4866;
    let t13587 = 1.0_f64 / t9 / t2621;
    let t13588 = t7 * t13587;
    let t13589 = t118 * t13588;
    (t13577, t13578, t13583, t13584, t13589)
}

//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1301/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1301(t16685: f64, t5662: f64, t4170: f64, t5661: f64, t11670: f64, t540: f64, t1017: f64, t86: f64, t11418: f64, t556: f64, t1650: f64, t2642: f64) -> (f64, f64, f64, f64) {
    let t16686 = t5662 * t16685;
    let t16687 = t4170 * t16686;
    let t16688 = t5661 * t16687;
    let t16690 = t11670 * t540;
    let t16692 = t86 * t1017 * t16690;
    let t16693 = t556 * t11418;
    let t16694 = t1650 * t2642;
    (t16688, t16692, t16693, t16694)
}

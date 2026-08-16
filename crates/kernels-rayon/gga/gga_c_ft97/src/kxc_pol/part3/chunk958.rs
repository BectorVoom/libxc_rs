//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 958/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk958(t113: f64, t4375: f64, t1274: f64, t332: f64, t992: f64, t4380: f64, t1578: f64, t505: f64, t5479: f64, t4376: f64, t4635: f64, t910: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t18794 = t113 * t4375;
    let t18795 = t1274 * t18794;
    let t18798 = t332 * t992;
    let t18799 = t4380 * t18798;
    let t18802 = t1274 * t1578;
    let t18804 = t5479 * t505;
    let t18809 = t4376 * t992;
    let t18812 = t910 * t4635;
    (t18795, t18799, t18802, t18804, t18809, t18812)
}

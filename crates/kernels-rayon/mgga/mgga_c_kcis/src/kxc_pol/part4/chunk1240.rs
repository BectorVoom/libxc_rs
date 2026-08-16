//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1240/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1240(t3255: f64, t5460: f64, t5465: f64, t544: f64, t5481: f64, t1319: f64, t5457: f64, t3809: f64, t5458: f64, t11633: f64, t1961: f64, t3762: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t16001 = 0.98556445e-3_f64 * t3255 * t5460;
    let t16003 = 0.19711289e-2_f64 * t3255 * t5465;
    let t16004 = t544 * t5481;
    let t16005 = t16004 * t1319;
    let t16006 = t5457 * t16005;
    let t16009 = t5458 * t3809;
    let t16010 = t5457 * t16009;
    let t16013 = t11633 * t1961;
    let t16014 = t16013 * t3762;
    (t16001, t16003, t16005, t16006, t16009, t16010, t16014)
}

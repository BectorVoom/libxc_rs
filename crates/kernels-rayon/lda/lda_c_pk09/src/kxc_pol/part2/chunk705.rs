//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 705/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk705(t481: f64, t6805: f64, t6700: f64, t68: f64, t1800: f64, t142: f64, t1991: f64, t902: f64, t92: f64, t6525: f64, t1947: f64, t1905: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t6806 = t481 * t6805;
    let t6810 = t6700 * t68;
    let t6811 = t6810 * t1800;
    let t6813 = t1991 * t142;
    let t6814 = t92 * t902;
    let t6816 = t6813 * t6814 * t6525;
    let t6818 = t1947 * t142;
    let t6822 = t1905 * t6525;
    (t6806, t6811, t6813, t6814, t6816, t6818, t6822)
}

//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1235/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1235(t26803: f64, t2822: f64, t27009: f64, t3500: f64, t7788: f64, t46978: f64, t7795: f64, t92748: f64, t26672: f64, t2865: f64, t380: f64, t283: f64, t3177: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t92872 = t2822 * t26803;
    let t92890 = t7788 * t3500 * t27009;
    let t92896 = t7788 * t46978 * t7795;
    let t92898 = t7788 * t92748;
    let t92908 = t2822 * t26672;
    let t92910 = t380 * t2865;
    let t92917 = t3177 * t283;
    (t92872, t92890, t92896, t92898, t92908, t92910, t92917)
}

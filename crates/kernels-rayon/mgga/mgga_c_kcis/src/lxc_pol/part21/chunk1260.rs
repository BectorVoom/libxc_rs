//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1260/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1260(t1183: f64, t26929: f64, t5182: f64, t14781: f64, t283: f64, t26897: f64, t5048: f64, t92522: f64, t26891: f64, t28050: f64, t14768: f64, t7748: f64) -> (f64, f64, f64, f64, f64) {
    let t95349 = t1183 * t26929 * t5182;
    let t95351 = t14781 * t283;
    let t95352 = t95351 * t26897;
    let t95354 = t92522 * t5048;
    let t95356 = t26891 * t28050;
    let t95358 = t7748 * t14768;
    (t95349, t95352, t95354, t95356, t95358)
}

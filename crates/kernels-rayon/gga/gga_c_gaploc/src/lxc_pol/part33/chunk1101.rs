//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1101/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1101(t2679: f64, t7696: f64, t9800: f64, t2624: f64, t7383: f64, t1391: f64, t825: f64, t9850: f64, t5840: f64, t9890: f64, t2017: f64, t3295: f64) -> (f64, f64, f64, f64, f64) {
    let t28678 = 0.38342925953920749676e1_f64 * t9800 * t7696 * t2679;
    let t28681 = 0.19171462976960374838e1_f64 * t9800 * t2624 * t7383;
    let t28683 = t825 * t1391 * t9850;
    let t28714 = t5840 * t9890;
    let t28726 = 0.11928910296775344344e1_f64 * t825 * t2017 * t3295;
    (t28678, t28681, t28683, t28714, t28726)
}

//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1279/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1279(t1141: f64, t29024: f64, t1203: f64, t10498: f64, t5189: f64, t8064: f64, t20210: f64, t7740: f64, t27999: f64, t46026: f64, t63371: f64, t7743: f64) -> (f64, f64, f64, f64, f64) {
    let t100929 = t29024 * t1141;
    let t100930 = t100929 * t1203;
    let t100933 = 12.0_f64 * t10498 * t8064 * t5189;
    let t100936 = t7740 * t20210;
    let t100940 = 12.0_f64 * t46026 * t27999;
    let t100942 = 2.0_f64 * t63371 * t7743;
    (t100930, t100933, t100936, t100940, t100942)
}

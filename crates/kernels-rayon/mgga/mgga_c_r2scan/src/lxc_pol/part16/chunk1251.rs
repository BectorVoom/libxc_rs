//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1251/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1251(t3446: f64, t3453: f64, t9063: f64, t37481: f64, t37483: f64, t40485: f64, t42958: f64, t42962: f64, t42965: f64, t42969: f64, t42972: f64, t42976: f64, t43716: f64, t43720: f64, t43724: f64, t43728: f64, t43732: f64) -> f64 {
    let t43887 = t3446 * t3453 * t9063;
    let t43889 = t42958 + t42962 - t42965 - t42969 + t37481 - t42972 - t42976 - t43716 - 0.1951603679568577289e-3_f64 * t37483 + t43720 - t43724 - t43728 + 0.29810146462873361018e-2_f64 * t40485 + t43732 - 0.36021158228745895953e-3_f64 * t43887;
    t43889
}

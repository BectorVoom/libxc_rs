//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1088/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1088(t3552: f64, t6755: f64, t1142: f64, t19309: f64, t6767: f64, t19327: f64, t1114: f64, t23040: f64, t6897: f64, t3492: f64, t5086: f64, t37358: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t38958 = t6755 * t3552;
    let t38961 = t19309 * t1142;
    let t38971 = t6767 * t3552;
    let t38976 = t19327 * t1142;
    let t39010 = t23040 * t1114;
    let t39030 = t1114 * t6897;
    let t39040 = t5086 * t3492;
    let t39046 = 0.26021382394247697185e-3_f64 * t37358;
    (t38958, t38961, t38971, t38976, t39010, t39030, t39040, t39046)
}

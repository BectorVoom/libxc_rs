//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1283/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1283(t3671: f64, t61033: f64, t61051: f64, t1381: f64, t61050: f64, t61063: f64, t1369: f64, t61062: f64, t17974: f64, t3689: f64, t1385: f64, t61086: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t63928 = t61033 * t3671;
    let t63935 = 119.0_f64 / 3456.0_f64 * t61051;
    let t63945 = t61050 * t1381;
    let t63949 = 35.0_f64 / 108.0_f64 * t61063;
    let t63957 = t61062 * t1369;
    let t63960 = t17974 * t3689;
    let t63964 = t61086 * t1385;
    (t63928, t63935, t63945, t63949, t63957, t63960, t63964)
}

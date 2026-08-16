//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 350/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk350(t1317: f64, t1319: f64, t1305: f64, t453: f64) -> (f64, f64, f64) {
    let t1320 = t1317 * t1319;
    let t1322 = 0.29896666666666666667e0_f64 * t1305;
    let t1324 = f64::sqrt(t453);
    (t1320, t1322, t1324)
}

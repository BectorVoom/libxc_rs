//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 350/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk350(t1319: f64, t1324: f64, t250: f64, t324: f64, t461: f64, t251: f64, t494: f64) -> (f64, f64, f64, f64) {
    let t1325 = t1324 * t1319;
    let t1328 = t250 * t324 * t461;
    let t1329 = 0.82156666666666666667e-1_f64 * t1328;
    let t1330 = t251 * t494;
    (t1325, t1328, t1329, t1330)
}

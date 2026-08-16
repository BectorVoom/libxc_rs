//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1344/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1344(t22338: f64, t28629: f64, t28594: f64, t5932: f64, t28640: f64, t5910: f64, t1468: f64, t22427: f64, t27517: f64, t29470: f64, t16622: f64, t27543: f64, t6012: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t102991 = t28629 * t22338;
    let t102993 = t28594 * t5932;
    let t102995 = t28640 * t5910;
    let t102997 = t1468 * t22427;
    let t102999 = t27517 * t29470;
    let t103002 = t16622 * t27543 * t6012;
    (t102991, t102993, t102995, t102997, t102999, t103002)
}

//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1171/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1171(t29638: f64, t29651: f64, t1629: f64, t29415: f64, t29417: f64, t29418: f64, t29419: f64, t29423: f64, t29426: f64, t29429: f64, t29432: f64, t29488: f64, t29624: f64, t633: f64) -> (f64, f64) {
    let t29652 = t29638 + t29651;
    let t29654 = -t1629 * t29652 + t29624 * t633 + t29415 - t29417 + t29418 + t29419 + t29423 + t29426 - t29429 - t29432 + t29488;
    (t29652, t29654)
}

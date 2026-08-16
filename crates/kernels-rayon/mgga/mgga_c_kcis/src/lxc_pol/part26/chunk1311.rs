//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1311/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1311(t1466: f64, t7193: f64, t1490: f64, t303: f64, t6928: f64, t94453: f64, t22219: f64, t7931: f64, t22224: f64, t553: f64, t1459: f64, t7262: f64) -> (f64, f64, f64, f64, f64) {
    let t102462 = t7193 * t1466;
    let t102464 = t303 * t102462 * t1490;
    let t102467 = t303 * t94453 * t6928;
    let t102475 = t303 * t7931 * t22219;
    let t102478 = t303 * t553 * t22224;
    let t102481 = t303 * t1459 * t7262;
    (t102464, t102467, t102475, t102478, t102481)
}

//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1263/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1263(t28500: f64, t4142: f64, t1394: f64, t27427: f64, t28499: f64, t27475: f64, t303: f64, t5633: f64, t1459: f64, t5757: f64, t28423: f64, t7895: f64) -> (f64, f64, f64, f64, f64) {
    let t98637 = t4142 * t28500;
    let t98640 = t1394 * t28499 * t27427;
    let t98643 = t303 * t27475 * t5633;
    let t98646 = t303 * t1459 * t5757;
    let t98649 = 0.46336805555555555556e-3_f64 * t7895 * t28423;
    (t98637, t98640, t98643, t98646, t98649)
}

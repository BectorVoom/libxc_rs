//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1041/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1041(t27387: f64, t4165: f64, t1394: f64, t3955: f64, t7931: f64, t303: f64, t1386: f64, t3999: f64, t4001: f64, t6176: f64, t3723: f64, t553: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t27388 = t27387 * t4165;
    let t27389 = t1394 * t27388;
    let t27391 = t7931 * t3955;
    let t27392 = t303 * t27391;
    let t27394 = t3999 * t1386;
    let t27395 = t27394 * t4001;
    let t27396 = t6176 * t27395;
    let t27399 = t553 * t3723;
    (t27388, t27389, t27391, t27392, t27395, t27396, t27399)
}

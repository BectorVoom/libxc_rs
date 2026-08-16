//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 480/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk480(t3463: f64, t1094: f64, t1164: f64, t3177: f64, t381: f64, t1242: f64, t1247: f64, t1241: f64, t68: f64, sigma0: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3464 = t3463 * sigma0;
    let t3473 = t1164 * t1094;
    let t3474 = t3473 * sigma0;
    let t3477 = t3177 * t381;
    let t3487 = t1242 * t1247;
    let t3489 = t1241 * t68;
    (t3464, t3473, t3474, t3477, t3487, t3489)
}

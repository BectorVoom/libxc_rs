//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 375/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk375(t1592: f64, t2256: f64, t1616: f64, t494: f64, t617: f64, t2194: f64) -> (f64, f64, f64) {
    let t2257 = t1592 * t2256;
    let t2259 = t494 * t617 * t1616;
    let t2260 = t2194 * t2259;
    (t2257, t2259, t2260)
}

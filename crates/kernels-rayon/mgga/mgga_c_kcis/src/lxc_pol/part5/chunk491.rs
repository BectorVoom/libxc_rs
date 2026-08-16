//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 491/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk491(t2051: f64, t585: f64, t2001: f64, t584: f64, t583: f64, t1546: f64, t487: f64, t579: f64, t488: f64, t251: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t2052 = t2051 * t585;
    let t2054 = t584 * t2001;
    let t2055 = t583 * t2054;
    let t2056 = t1546 * t2055;
    let t2058 = t579 * t487;
    let t2060 = 1.0_f64 / t488 / t2058;
    let t2061 = t2060 * t251;
    (t2052, t2054, t2055, t2056, t2060, t2061)
}

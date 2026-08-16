//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1235/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1235(t1928: f64, t3961: f64, t990: f64, t28372: f64, t4001: f64, t5885: f64, t28383: f64, t3728: f64, t1464: f64, t2046: f64, t27387: f64, t3954: f64) -> (f64, f64, f64, f64) {
    let t98155 = t3961 * t1928 * t990;
    let t98159 = t28372 * t5885 * t4001;
    let t98162 = t3728 * t28383;
    let t98166 = t1464 * t27387 * t2046 * t3954;
    (t98155, t98159, t98162, t98166)
}

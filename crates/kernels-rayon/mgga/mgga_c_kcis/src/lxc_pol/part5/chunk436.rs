//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 436/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk436(t1684: f64, t274: f64, t1664: f64, t1671: f64, t1674: f64, t1677: f64, t964: f64, t967: f64) -> (f64, f64) {
    let t1685 = t1684 * t274;
    let t1692 = 0.258925e1_f64 * t1671 - t964 - 0.301925e0_f64 * t1664 + 0.16504875e0_f64 * t1674 - t967 - 0.82785e-1_f64 * t1677;
    (t1685, t1692)
}

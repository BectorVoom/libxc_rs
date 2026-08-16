//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 526/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk526(t4: f64, t66: f64, t789: f64, t128: f64, t717: f64, t2440: f64, t2459: f64, t794: f64, t103: f64, t11: f64, t122: f64, t144: f64, t145: f64, t148: f64, t2477: f64, t2542: f64, t2546: f64, t2552: f64, t2555: f64, t2559: f64, t2561: f64, t745: f64, t784: f64, t788: f64, t791: f64, t795: f64, t85: f64) -> (f64, f64, f64, f64, f64) {
    let t2565 = t789 * t4 * t66;
    let t2568 = t128 * t717;
    let t2569 = t2568 * t2440;
    let t2572 = t794 * t2459;
    let t2584 = 0.619125e-2_f64 * t2542 * t145 - 0.24765e-1_f64 * t2546 * t791 - 0.123825e-1_f64 * t784 * t795 + 0.206375e-2_f64 * t2552 * t2555 + 0.24765e-1_f64 * t2559 * t2561 + 0.1651e-1_f64 * t788 * t2565 + 0.123825e-1_f64 * t144 * t2569 - 0.619125e-2_f64 * t144 * t2572 + 0.17687407407407407407e-1_f64 * t85 * t103 * t122 - 0.10612444444444444444e0_f64 * t85 * t11 * t745 - 0.79593333333333333331e-1_f64 * t85 * t148 * t2477;
    (t2565, t2568, t2569, t2572, t2584)
}

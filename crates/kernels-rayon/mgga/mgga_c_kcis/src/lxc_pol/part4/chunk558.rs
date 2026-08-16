//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 558/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk558(t2880: f64, t995: f64, t991: f64, t1004: f64, t25: f64, t285: f64, t335: f64) -> (f64, f64, f64, f64, f64) {
    let t2881 = t2880 * t995;
    let t2882 = t991 * t2881;
    let t2884 = t25 * t1004;
    let t2885 = t285 * t2884;
    let t2887 = 1.0_f64 / t335;
    (t2881, t2882, t2884, t2885, t2887)
}

//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1238/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1238<F: Float>(t123814: F, t1434: F, t193: F, t743: F, t1882: F, t31010: F, t108109: F, t2354: F, t446: F, t992: F, t18514: F, t96945: F, t27762: F, t6118: F, t122334: F, t24438: F, t27805: F) -> (F, F, F, F, F, F) {
    let t123817 = t1434 * t193 * t743 * t123814;
    let t123819 = t1882 * t31010;
    let t123823 = t446 * t2354 * t108109 * t992;
    let t123825 = t96945 * t18514;
    let t123827 = t6118 * t27762 * t123825;
    let t123830 = t27805 * t24438 * t122334;
    (t123817, t123819, t123823, t123825, t123827, t123830)
}

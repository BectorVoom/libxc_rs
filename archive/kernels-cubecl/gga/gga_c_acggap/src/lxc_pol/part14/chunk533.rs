//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 533/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk533<F: Float>(t3141: F, t3160: F, t19: F, t2066: F, t124: F, t1149: F, t329: F, t107: F, t2607: F, t2690: F, t4: F, t118: F) -> (F, F, F, F, F, F) {
    let t3588 = F::cast_from(0.38033333333333333333e1_f64) * t3141;
    let t3592 = F::cast_from(0.12225e1_f64) * t3160;
    let t3615 = t2066 * t19;
    let t3616 = t124 * t3615;
    let t3621 = t329 * t1149;
    let t3644 = -F::cast_from(0.12962962962962962963e0_f64) * t4 * t2607 * t107 - F::cast_from(0.40124259259259259261e-1_f64) * t2690;
    let t3645 = t3644 * t118;
    (t3588, t3592, t3616, t3621, t3644, t3645)
}

//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 515/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk515<F: Float>(t339: F, t3570: F, t366: F, t986: F, t374: F, t3106: F, t3109: F, t3141: F, t3160: F, t19: F, t2066: F, t124: F, t1149: F, t329: F, t107: F, t2607: F, t2690: F, t4: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t3571 = t3570 * t339;
    let t3573 = t986 * t366;
    let t3574 = t3573 * t374;
    let t3579 = 0.10866666666666666667e1 * t3106;
    let t3580 = 0.978e0 * t3109;
    let t3588 = 0.38033333333333333333e1 * t3141;
    let t3592 = 0.12225e1 * t3160;
    let t3615 = t2066 * t19;
    let t3616 = t124 * t3615;
    let t3621 = t329 * t1149;
    let t3644 = -0.12962962962962962963e0 * t4 * t2607 * t107 - 0.40124259259259259261e-1 * t2690;
    (t3571, t3573, t3574, t3579, t3580, t3588, t3592, t3616, t3621, t3644)
}

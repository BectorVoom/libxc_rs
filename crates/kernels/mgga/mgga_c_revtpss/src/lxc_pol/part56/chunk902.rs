//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 902/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk902<F: Float>(t8598: F, t9593: F, t11239: F, t13181: F, t3736: F, t1450: F, t8594: F, t4147: F, t211: F, t9644: F, t11006: F, t256: F, t2410: F, t11238: F, t196: F, t3800: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t37110 = t9593 * t8598;
    let t37880 = t11239 * t13181;
    let t37885 = t11239 * t3736;
    let t37956 = t8594 * t1450;
    let t37972 = t8598 * t4147;
    let t39643 = 1.0 / t9644 / t211;
    let t41077 = 1.0 / t11006 / t256;
    let t41153 = t2410 * t2410;
    let t41154 = 1.0 / t41153;
    let t42859 = 1.0 / t11238 / t196;
    let t44125 = t3800 * t3800;
    (t37110, t37880, t37885, t37956, t37972, t39643, t41077, t41154, t42859, t44125)
}

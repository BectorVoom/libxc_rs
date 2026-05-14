//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1116/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1116<F: Float>(t27543: F, t5747: F, t491: F, t5998: F, t1528: F, t1928: F, t4254: F, t572: F, t2060: F, t1467: F, t1532: F, t4188: F, t8182: F, t28450: F, t4142: F, t27376: F, t28392: F, sigma2: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t97767 = t5747 * t27543;
    let t97772 = t5998 * t491;
    let t97784 = t1528 * t1928;
    let t97793 = t4254 * t572;
    let t97800 = sigma2 * t2060;
    let t97801 = t1467 * t97800;
    let t97804 = t4254 * t1928;
    let t97821 = t1532 * t572;
    let t97991 = t8182 * t4188;
    let t97997 = t4142 * t28450;
    let t98016 = t28392 * t27376;
    (t97767, t97772, t97784, t97793, t97800, t97801, t97804, t97821, t97991, t97997, t98016)
}

//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 983/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk983<F: Float>(t22603: F, t22605: F, t22608: F, t22718: F, t22721: F, t22724: F, t22727: F, t22730: F, t22734: F, t22737: F, t22741: F, t10570: F, t10615: F, t10738: F, t10739: F, t15989: F, t15991: F, t15996: F, t16379: F, t16389: F, t16392: F, t16399: F, t16400: F, t22575: F, t22583: F, t22610: F, t22684: F, t22705: F, t22707: F, t22711: F, t22714: F, t22843: F) -> (F,) {
    let t22865 = 0.10954222222222222222e0 * t22718 - 0.49293999999999999999e0 * t22721 - 0.65725333333333333332e0 * t22724 + 0.16431333333333333333e0 * t22727 - 0.27385555555555555556e-1 * t22730 - 0.1898925e1 * t22605 - 0.9494625e0 * t22608 + 0.3071625e0 * t22734 + 0.15358125e0 * t22737 + 0.142419375e1 * t22603 - 0.76790625e-1 * t22741;
    let t22867 = -0.26574814814814814815e0 * t15989 - 0.39862222222222222222e0 * t15996 - 0.91285185185185185187e-1 * t10615 - 0.13287407407407407408e0 * t10570 + 0.13287407407407407407e0 * t15991 - t16379 - t10738 - 0.18257037037037037037e0 * t16389 - 0.21908444444444444444e0 * t16392 + 0.16431333333333333333e0 * t22684 + t22843 - 0.19931111111111111111e0 * t22575 + 0.99655555555555555557e-1 * t22583 - 0.10954222222222222222e0 * t22705 + 0.54771111111111111111e-1 * t22707 - t16399 + 0.36514074074074074073e-1 * t16400 - t10739 + 0.1898925e1 * t22610 + 0.3071625e0 * t22711 - 0.36514074074074074075e-1 * t22714 + t22865;
    (t22867,)
}

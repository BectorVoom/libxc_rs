//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 928/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk928<F: Float>(t16034: F, t1653: F, t10739: F, t16032: F, t16410: F, t16413: F, t16416: F, t16419: F, t16421: F, t16424: F, t16427: F, t16432: F, t10570: F, t10607: F, t10615: F, t10617: F, t10619: F, t10738: F, t15989: F, t15996: F, t16028: F, t16045: F, t16048: F, t16068: F, t16070: F, t16386: F, t16389: F, t16392: F, t16396: F, t16399: F, t16400: F, t16403: F, t16406: F) -> (F, F) {
    let t16434 = t1653 * t16034;
    let t16436 = 0.10954222222222222222e0 * t16410 - t10739 + 0.16431333333333333333e0 * t16413 - 0.49293999999999999999e0 * t16416 + 0.3071625e0 * t16419 + 0.15358125e0 * t16421 + 0.16431333333333333333e0 * t16424 - 0.65725333333333333332e0 * t16427 + 0.59793333333333333334e0 * t16032 + 0.16431333333333333333e0 * t16432 + 0.3071625e0 * t16434;
    let t16438 = 0.1898925e1 * t16045 - 0.1898925e1 * t16068 - 0.9494625e0 * t16070 - 0.13287407407407407408e0 * t15989 - 0.21924222222222222222e1 * t15996 - 0.10954222222222222222e0 * t10607 - 0.18257037037037037037e0 * t10615 + 0.54771111111111111111e-1 * t10617 + 0.18257037037037037037e-1 * t10619 - 0.26574814814814814816e0 * t10570 + t16386 + 0.59793333333333333334e0 * t16028 - t10738 - 0.91285185185185185185e-1 * t16389 - 0.71202444444444444443e0 * t16392 + 0.142419375e1 * t16048 - 0.76790625e-1 * t16396 - t16399 + 0.36514074074074074074e-1 * t16400 - 0.27385555555555555556e-1 * t16403 - 0.36514074074074074075e-1 * t16406 + t16436;
    (t16434, t16438)
}

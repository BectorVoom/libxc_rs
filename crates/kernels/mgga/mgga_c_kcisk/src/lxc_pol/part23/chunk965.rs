//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 965/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk965<F: Float>(t14827: F, t5793: F, t13023: F, t2092: F, t3640: F, t13021: F, t13048: F, t5720: F, t12905: F, t5753: F, t1190: F, t5749: F, t3639: F, t2093: F, t3671: F, t5752: F) -> (F, F, F, F, F, F, F) {
    let t19594 = t5793 * t14827;
    let t19599 = t2092 * t13023;
    let t19600 = t19599 * t3640;
    let t19602 = 0.51725014705706168417e3 * t13021 * t19600;
    let t19604 = 4.0 * t13048 * t5720;
    let t19606 = 0.32163648644302209644e2 * t12905 * t5753;
    let t19607 = t5749 * t1190;
    let t19609 = 4.0 * t3639 * t19607;
    let t19610 = t2093 * t3671;
    let t19612 = 2.0 * t3639 * t19610;
    let t19613 = t5752 * t3640;
    (t19594, t19602, t19604, t19606, t19609, t19612, t19613)
}

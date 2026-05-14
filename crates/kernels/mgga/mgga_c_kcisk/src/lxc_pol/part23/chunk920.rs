//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 920/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk920<F: Float>(t3743: F, t5886: F, t1411: F, t14223: F, t2237: F, t3739: F, t5982: F, t13401: F, t2231: F, t1415: F, t2233: F, t5976: F, t12951: F, t470: F, t2059: F, t3278: F) -> (F, F, F, F, F, F, F, F, F) {
    let t19017 = t5886 * t3743;
    let t19018 = t1411 * t19017;
    let t19020 = t14223 * t2237;
    let t19022 = t3739 * t5982;
    let t19024 = t13401 * t2231;
    let t19025 = t1415 * t19024;
    let t19026 = t1411 * t19025;
    let t19028 = t14223 * t2233;
    let t19030 = t3739 * t5976;
    let t19032 = t470 * t12951;
    let t19033 = t2059 * t3278;
    (t19018, t19020, t19022, t19024, t19026, t19028, t19030, t19032, t19033)
}

//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1040/1221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1040<F: Float>(t7784: F, t8083: F, t20572: F, t27028: F, t5329: F, t1267: F, t1851: F, t26996: F, t251: F, t5345: F, t1250: F) -> (F, F, F, F, F, F, F, F) {
    let t28176 = t8083 * t7784;
    let t28178 = t27028 * t20572;
    let t28179 = t5329 * t28178;
    let t28182 = t1851 * t1267;
    let t28183 = t26996 * t28182;
    let t28184 = t5329 * t28183;
    let t28189 = t5345 * t251;
    let t28190 = t28189 * t1250;
    (t28176, t28178, t28179, t28182, t28183, t28184, t28189, t28190)
}

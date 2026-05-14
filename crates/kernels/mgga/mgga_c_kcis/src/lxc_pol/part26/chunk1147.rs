//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1147/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1147<F: Float>(t1394: F, t28499: F, t5655: F, t4153: F, t5663: F, t18210: F, t29513: F, t7978: F, t1464: F, t2011: F, t27387: F, t52073: F, t23157: F, t7977: F, t28360: F, t98470: F) -> (F, F, F, F, F, F, F) {
    let t101938 = t1394 * t28499 * t5655;
    let t101941 = t4153 * t28499 * t5663;
    let t101943 = t18210 * t29513;
    let t101944 = t7978 * t101943;
    let t101948 = t1464 * t27387 * t52073 * t2011;
    let t101950 = t7977 * t23157;
    let t101954 = t1464 * t98470 * t28360;
    (t101938, t101941, t101943, t101944, t101948, t101950, t101954)
}

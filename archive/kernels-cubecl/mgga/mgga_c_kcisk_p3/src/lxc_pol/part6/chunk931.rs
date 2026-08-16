//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 931/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk931<F: Float>(t1935: F, t29528: F, t17821: F, t9051: F, t28950: F, t747: F, t746: F, t1948: F, t7320: F, t8972: F, t29494: F, t29496: F, t29499: F, t29501: F, t29505: F, t29507: F, t29514: F, t29517: F, t29520: F, t29524: F, t29526: F) -> (F, F, F, F, F) {
    let t29529 = t1935 * t29528;
    let t29531 = t17821 * t9051;
    let t29533 = t747 * t28950;
    let t29534 = t746 * t29533;
    let t29535 = t1948 * t29534;
    let t29537 = t7320 * t8972;
    let t29539 = t29494 / F::cast_from(16.0_f64) + t29496 / F::cast_from(6.0_f64) - t29499 / F::cast_from(3.0_f64) - t29501 / F::cast_from(4.0_f64) - t29505 / F::cast_from(16.0_f64) + F::cast_from(11.0_f64) / F::cast_from(6.0_f64) * t29507 + F::cast_from(209.0_f64) / F::cast_from(216.0_f64) * t29514 + t29517 / F::cast_from(4.0_f64) + t29520 / F::cast_from(36.0_f64) + t29524 / F::cast_from(864.0_f64) - t29526 / F::cast_from(24.0_f64) - F::cast_from(11.0_f64) / F::cast_from(6.0_f64) * t29529 - F::cast_from(3.0_f64) / F::cast_from(128.0_f64) * t29531 + t29535 / F::cast_from(256.0_f64) + F::cast_from(3.0_f64) / F::cast_from(256.0_f64) * t29537;
    (t29529, t29531, t29535, t29537, t29539)
}

//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 655/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk655<F: Float>(t741: F, t9082: F, t641: F, t8786: F, t746: F, t719: F, t8672: F, t735: F, t5284: F, t9048: F, t9052: F, t9056: F, t9059: F, t9063: F, t9067: F, t9070: F, t9073: F, t9080: F) -> (F, F, F, F, F, F, F, F) {
    let t9083 = t741 * t9082;
    let t9085 = t641 * t8786;
    let t9086 = t746 * t9085;
    let t9087 = t741 * t9086;
    let t9089 = t719 * t8672;
    let t9090 = t735 * t9089;
    let t9091 = t5284 * t9090;
    let t9093 = -t9048 / F::cast_from(12.0_f64) - t9052 / F::cast_from(128.0_f64) + F::cast_from(11.0_f64) / F::cast_from(18.0_f64) * t9056 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t9059 - t9063 / F::cast_from(256.0_f64) - t9067 / F::cast_from(576.0_f64) - t9070 / F::cast_from(24.0_f64) + t9073 / F::cast_from(96.0_f64) - F::cast_from(19.0_f64) / F::cast_from(144.0_f64) * t9080 + t9083 / F::cast_from(18.0_f64) - t9087 / F::cast_from(192.0_f64) + t9091 / F::cast_from(8.0_f64);
    (t9083, t9085, t9086, t9087, t9089, t9090, t9091, t9093)
}

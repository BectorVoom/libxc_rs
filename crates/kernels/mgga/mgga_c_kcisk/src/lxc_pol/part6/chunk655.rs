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
    let t9093 = -t9048 / F::new(12.0) - t9052 / F::new(128.0) + F::new(11.0) / F::new(18.0) * t9056 - F::new(2.0) / F::new(9.0) * t9059 - t9063 / F::new(256.0) - t9067 / F::new(576.0) - t9070 / F::new(24.0) + t9073 / F::new(96.0) - F::new(19.0) / F::new(144.0) * t9080 + t9083 / F::new(18.0) - t9087 / F::new(192.0) + t9091 / F::new(8.0);
    (t9083, t9085, t9086, t9087, t9089, t9090, t9091, t9093)
}

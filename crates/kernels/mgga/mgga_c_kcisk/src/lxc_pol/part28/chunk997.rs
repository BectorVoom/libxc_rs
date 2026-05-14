//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 997/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk997<F: Float>(t6746: F, t6790: F, t1060: F, t8537: F, t5101: F, t8536: F, t1824: F, t696: F, t8494: F, t1856: F, t22484: F, t1835: F, t22387: F, t22488: F, t22392: F, t706: F) -> (F, F, F, F, F, F, F, F) {
    let t23108 = t6746 * t6790;
    let t23111 = t8537 * t1060;
    let t23114 = t5101 * t8536;
    let t23115 = t23114 * t1824;
    let t23118 = t696 * t8494;
    let t23134 = t1856 * t22484;
    let t23137 = t1835 * t22387;
    let t23140 = t1856 * t22488;
    let t23143 = t706 * t22392;
    (t23108, t23111, t23115, t23118, t23134, t23137, t23140, t23143)
}

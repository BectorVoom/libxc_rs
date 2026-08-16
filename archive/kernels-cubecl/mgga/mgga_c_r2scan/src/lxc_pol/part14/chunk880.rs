//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 880/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk880<F: Float>(t571: F, t8021: F, t6240: F, t928: F, t259: F, t6360: F, t2572: F, t7518: F, t360: F, t1570: F, t1554: F, t2124: F, t2545: F) -> (F, F, F, F, F, F, F, F) {
    let t8022 = t571 * t8021;
    let t8026 = t6240 * t928;
    let t8028 = t6360 * t259;
    let t8029 = t571 * t8028;
    let t8030 = t2572 * t7518;
    let t8031 = t360 * t8030;
    let t8034 = t2572 * t1570;
    let t8035 = t360 * t8034;
    let t8039 = t2124 * t2545 * t1554;
    (t8022, t8026, t8029, t8030, t8031, t8034, t8035, t8039)
}

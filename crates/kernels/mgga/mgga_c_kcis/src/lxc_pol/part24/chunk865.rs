//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 865/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk865<F: Float>(t10583: F, t3399: F, t6272: F, t1154: F, t14915: F, t1646: F, t330: F, t6478: F, t829: F, t10544: F, t1110: F, t1115: F, t1143: F, t1153: F, t14940: F, t14956: F, t14959: F, t1757: F, t1761: F, t1780: F, t18547: F, t18551: F, t18740: F, t18858: F, t3381: F, t365: F, t4626: F, t5102: F, t5122: F, t6593: F, t6605: F, t6641: F) -> (F,) {
    let t20076 = t3399 * t10583 * t6272;
    let t20080 = t1154 * t14915 * t1646;
    let t20084 = t6478 * t330;
    let t20086 = t1154 * t20084 * t829;
    let t20093 = -0.619125e-2 * t1143 * t6605 - 0.619125e-2 * t365 * t18858 - 0.232171875e-2 * t14940 * t18740 - 0.619125e-2 * t6641 * t1115 + 0.1857375e-1 * t5102 * t1757 + 0.1857375e-1 * t1780 * t4626 - 0.123825e-1 * t5102 * t1761 + 0.9286875e-2 * t6641 * t1110 - 0.1857375e-1 * t10544 * t6593 - 0.44218518518518518518e-1 * t1153 * t20076 - 0.53062222222222222222e-1 * t1153 * t20080 + 0.70749629629629629628e-1 * t14956 - t14959 - 0.26531111111111111111e-1 * t1153 * t20086 + 0.1857375e-1 * t3381 * t18547 - 0.371475e-1 * t5122 * t18551;
    (t20093,)
}

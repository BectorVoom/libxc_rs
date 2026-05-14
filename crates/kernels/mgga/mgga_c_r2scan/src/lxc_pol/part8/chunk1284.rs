//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1284/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1284<F: Float>(t3053: F, t560: F, t6085: F, t6086: F, t1632: F, t549: F, t551: F, t9365: F, t25299: F, t7619: F, t20762: F, t2207: F, t2208: F, t9268: F, t1610: F, t5103: F, t9399: F) -> (F, F, F, F, F, F) {
    let t30140 = t3053 * t560;
    let t30142 = t6085 * t6086 * t30140;
    let t30146 = t549 * t551 * t1632 * t9365;
    let t30158 = t25299 * t7619;
    let t30159 = t20762 * t30158;
    let t30165 = t2207 * t9268 * t2208;
    let t30168 = t5103 * t1610 * t9399;
    (t30142, t30146, t30158, t30159, t30165, t30168)
}

//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1290/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1290<F: Float>(t1568: F, t20659: F, t3055: F, t6191: F, t3071: F, t6212: F, t20664: F, t6211: F, t7555: F, t8240: F, t29946: F, t6086: F, t6535: F, t22744: F, t8757: F, t2667: F, t7387: F) -> (F, F, F, F, F, F) {
    let t30426 = t6191 * t1568 * t3055 * t20659;
    let t30428 = t6212 * t3071;
    let t30430 = t20664 * t6211 * t30428;
    let t30437 = t8240 * t7555;
    let t30446 = t6535 * t6086 * t29946;
    let t30448 = t22744 * t8757;
    let t30456 = t2667 * t7387;
    (t30426, t30430, t30437, t30446, t30448, t30456)
}

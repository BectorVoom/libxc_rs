//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1022/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1022<F: Float>(t2593: F, t9053: F, t2150: F, t755: F, t8750: F, t2484: F, t26550: F, t26527: F, t9042: F, t26553: F, t815: F, t808: F, t9046: F, t2490: F, t62: F, t9047: F) -> (F, F, F, F, F, F, F) {
    let t91811 = t2593 * t9053;
    let t91814 = t755 * t2150 * t8750;
    let t91816 = t2484 * t26550;
    let t91818 = t9042 * t26527;
    let t91820 = t815 * t26553;
    let t91822 = t808 * t9046;
    let t91825 = t2490 * t62 * t9047;
    (t91811, t91814, t91816, t91818, t91820, t91822, t91825)
}

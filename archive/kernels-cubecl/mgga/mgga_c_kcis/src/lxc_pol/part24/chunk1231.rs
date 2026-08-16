//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1231/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1231<F: Float>(t15216: F, t29122: F, t26960: F, t20330: F, t5310: F, t922: F, t1262: F, t26996: F, t5329: F, t6842: F, t1020: F, t26753: F, t6625: F) -> (F, F, F, F, F) {
    let t100074 = t15216 * t29122;
    let t100075 = t26960 * t100074;
    let t100078 = t5310 * t20330 * t922;
    let t100090 = t5329 * t26996 * t6842 * t1262;
    let t100094 = t1020 * t26753 * t6625;
    (t100074, t100075, t100078, t100090, t100094)
}

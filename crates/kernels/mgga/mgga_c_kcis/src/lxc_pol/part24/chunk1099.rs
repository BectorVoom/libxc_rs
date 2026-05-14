//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1099/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1099<F: Float>(t20671: F, t5329: F, t7773: F, t283: F, t6708: F, t1020: F, t7719: F, t1267: F, t28110: F, t5310: F, t6276: F, t15216: F, t29122: F, t26960: F, t20330: F, t922: F) -> (F, F, F, F, F, F) {
    let t100056 = t5329 * t7773 * t20671;
    let t100059 = t6708 * t283;
    let t100061 = t1020 * t100059 * t7719;
    let t100067 = t5310 * t28110 * t6276 * t1267;
    let t100074 = t15216 * t29122;
    let t100075 = t26960 * t100074;
    let t100078 = t5310 * t20330 * t922;
    (t100056, t100061, t100067, t100074, t100075, t100078)
}

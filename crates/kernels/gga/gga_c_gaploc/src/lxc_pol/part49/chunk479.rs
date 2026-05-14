//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 479/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk479<F: Float>(t1339: F, t2754: F, t1: F, t8025: F, t544: F, t188: F, t7937: F, t7887: F, t1415: F, t2967: F, t747: F, t2925: F, t835: F, t2936: F, t769: F, t2089: F) -> (F, F, F, F, F, F, F, F, F) {
    let t8272 = t1339 * t2754;
    let t8330 = t8025 * t1;
    let t8331 = t544 * t8330;
    let t8352 = t188 * t7937;
    let t8410 = t7887 * t1;
    let t8411 = t1415 * t8410;
    let t8440 = t2967 * t747;
    let t8469 = t835 * t2925;
    let t8478 = t769 * t2936;
    let t8483 = t2089 * t2925;
    (t8272, t8331, t8352, t8410, t8411, t8440, t8469, t8478, t8483)
}

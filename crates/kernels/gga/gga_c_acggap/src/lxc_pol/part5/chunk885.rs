//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 885/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk885<F: Float>(t174: F, t5079: F, t435: F, t13080: F, t4925: F, t12747: F, t1466: F, t1165: F, t3194: F, t4289: F, t5284: F, t14176: F, t4967: F, t1567: F, t4210: F, t3216: F, t4360: F) -> (F, F, F, F, F, F, F, F) {
    let t15560 = t174 * t5079;
    let t15565 = t435 * t5079;
    let t15574 = t13080 * t4925;
    let t15576 = t12747 * t1466;
    let t15610 = t3194 * t1165 * t4289 * t5284;
    let t15622 = t14176 * t4967;
    let t15626 = t3194 * t1165 * t1567 * t4210;
    let t15628 = t3216 * t4360;
    (t15560, t15565, t15574, t15576, t15610, t15622, t15626, t15628)
}

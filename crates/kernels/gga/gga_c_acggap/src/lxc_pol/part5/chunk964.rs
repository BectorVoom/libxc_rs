//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 964/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk964<F: Float>(t1160: F, t1539: F, t545: F, t943: F, t4167: F, t4180: F, t377: F, t4251: F, t3073: F, t5315: F, t945: F, t4146: F, t4162: F, t4166: F, t15758: F, t1629: F, t3088: F) -> (F, F, F, F, F, F, F) {
    let t18989 = t1160 * t545 * t943 * t1539;
    let t19000 = t4180 * t4167;
    let t19005 = t377 * t4251;
    let t19015 = t3073 * t5315 * t945;
    let t19023 = t1160 * t4146 * t4162;
    let t19026 = t1160 * t4166 * t4162;
    let t19029 = t3088 * t1629 * t15758;
    (t18989, t19000, t19005, t19015, t19023, t19026, t19029)
}

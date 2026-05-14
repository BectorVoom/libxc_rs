//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 942/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk942<F: Float>(t3375: F, t4326: F, t12727: F, t1549: F, t16183: F, t2059: F, t4264: F, t1181: F, t12991: F, t3355: F, t535: F, t1164: F, t4847: F, t1446: F, t3228: F, t1352: F, t3244: F) -> (F, F, F, F, F, F, F, F) {
    let t17946 = t3375 * t4326;
    let t17948 = t12727 * t1549;
    let t17951 = t16183 * t2059;
    let t17952 = t17951 * t4264;
    let t17962 = t12991 * t1181 * t535 * t3355;
    let t17972 = t1164 * t4847;
    let t17984 = t3228 * t1446;
    let t18000 = t3244 * t1352;
    (t17946, t17948, t17951, t17952, t17962, t17972, t17984, t18000)
}

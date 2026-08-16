//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 684/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk684<F: Float>(t10964: F, t813: F, t10783: F, t1457: F, t2194: F, t3484: F, t8528: F, t935: F, t1445: F, t3477: F, t5771: F, t10713: F) -> (F, F, F, F, F, F) {
    let t10966 = F::cast_from(0.61348681526273199483e1_f64) * t813 * t10964;
    let t10967 = t1457 * t10783;
    let t10971 = F::cast_from(0.46011511144704899612e1_f64) * t2194 * t3484;
    let t10972 = t8528 * t935;
    let t10973 = t1445 * t10972;
    let t10975 = F::cast_from(0.46011511144704899612e1_f64) * t813 * t10973;
    let t10977 = F::cast_from(0.71500979903700853338e0_f64) * t5771 * t3477;
    let t10978 = t1457 * t10713;
    (t10966, t10967, t10971, t10975, t10977, t10978)
}

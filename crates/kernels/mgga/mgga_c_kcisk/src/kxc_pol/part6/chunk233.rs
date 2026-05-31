//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 233/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk233<F: Float>(t1008: F, t195: F, t196: F, t852: F, t179: F, t60: F, t15: F, t183: F, t2: F, t142: F, t4: F, t151: F, t181: F) -> (F, F, F, F, F, F, F, F) {
    let t1009 = F::cast_from(1.0_f64) / t1008;
    let t1010 = t195 * t1009;
    let t1011 = t852 * t196;
    let t1014 = t60 * t179;
    let t1015 = t1014 * t15;
    let t1016 = t183 * t2;
    let t1018 = t1016 * t4 * t142;
    let t1021 = t181 * t151;
    (t1009, t1010, t1011, t1014, t1015, t1016, t1018, t1021)
}

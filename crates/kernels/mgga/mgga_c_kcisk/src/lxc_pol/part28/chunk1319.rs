//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1319/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1319<F: Float>(t1693: F, t1772: F, t7201: F, t34100: F, t5074: F, t33021: F, t5014: F, t654: F, t6973: F, t32955: F, t34073: F, t116116: F, t9649: F, t1782: F, t658: F, t10463: F, t1791: F) -> (F, F, F, F, F, F, F, F, F) {
    let t116201 = t1693 * t7201 * t1772;
    let t116210 = t5074 * t34100;
    let t116211 = 0.22109259259259259258e-2 * t116210;
    let t116212 = t5014 * t33021;
    let t116223 = t6973 * t654;
    let t116245 = t34073 * t32955;
    let t116289 = 0.26805555555555555556e-2 * t9649 * t116116;
    let t116304 = t658 * t1782;
    let t116311 = t1791 * t10463;
    (t116201, t116210, t116211, t116212, t116223, t116245, t116289, t116304, t116311)
}

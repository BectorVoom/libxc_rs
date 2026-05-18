//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 805/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk805<F: Float>(t181: F, t3086: F, t3088: F, t955: F, t21: F, t3117: F, t3201: F, t142: F, t3107: F, t5: F, t1016: F, t4: F, t918: F) -> (F, F, F, F, F) {
    let t12434 = t181 * t3086;
    let t12435 = t3088 * t955;
    let t12436 = t12434 * t12435;
    let t12442 = t3201 * t21 * t3117;
    let t12446 = t5 * t142 * t3107;
    let t12450 = t1016 * t4 * t918;
    (t12435, t12436, t12442, t12446, t12450)
}

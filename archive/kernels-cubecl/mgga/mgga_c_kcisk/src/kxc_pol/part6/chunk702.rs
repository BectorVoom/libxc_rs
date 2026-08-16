//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 702/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk702<F: Float>(t2921: F, t846: F, t2912: F, t2918: F, t157: F, t2903: F, t2856: F, t2879: F, t831: F, t32: F, t5: F, t969: F) -> (F, F, F, F) {
    let t12572 = t2921 * t846;
    let t12573 = t2918 * t2912 * t12572;
    let t12576 = t157 * t2903;
    let t12581 = F::cast_from(6.0_f64) * t2856 * t831 * t2879;
    let t12584 = F::cast_from(0.34451131037037037036e-2_f64) * t5 * t969 * t32;
    (t12573, t12576, t12581, t12584)
}

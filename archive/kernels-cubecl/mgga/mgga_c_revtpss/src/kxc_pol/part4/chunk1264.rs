//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1264/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk1264<F: Float>(t3111: F, t4834: F, t1062: F, t11788: F, t3105: F, t3204: F, t11262: F, t1670: F, t1041: F, t3172: F, t4824: F, t3127: F) -> (F, F, F, F, F) {
    let t15724 = F::cast_from(0.19055119163586549765e-3_f64) * t4834 * t3111;
    let t15725 = t11788 * t1062;
    let t15728 = t3204 * t3105;
    let t15731 = t11262 * t1670;
    let t15732 = t1041 * t15731;
    let t15734 = t3172 * t4824;
    let t15736 = F::cast_from(0.19055119163586549765e-3_f64) * t3127 * t15734;
    (t15724, t15725, t15728, t15732, t15736)
}

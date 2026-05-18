//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1102/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk1102<F: Float>(t1062: F, t11940: F, t3111: F, t4834: F, t11262: F, t1670: F, t1041: F, t3172: F, t4824: F, t3127: F, t3211: F, t4845: F) -> (F, F, F, F, F) {
    let t15716 = t11940 * t1062;
    let t15724 = F::new(0.19055119163586549765e-3) * t4834 * t3111;
    let t15731 = t11262 * t1670;
    let t15732 = t1041 * t15731;
    let t15734 = t3172 * t4824;
    let t15736 = F::new(0.19055119163586549765e-3) * t3127 * t15734;
    let t15744 = F::new(0.15244095330869239812e-2) * t3211 * t4845;
    (t15716, t15724, t15732, t15736, t15744)
}

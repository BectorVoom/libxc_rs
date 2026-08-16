//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 936/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk936<F: Float>(t2623: F, t815: F, t2588: F, t2597: F, t2526: F, t823: F, t755: F, t774: F, t7624: F, t808: F, t2484: F, t2615: F) -> (F, F, F, F, F, F) {
    let t9026 = t815 * t2623;
    let t9028 = t2588 * t2597;
    let t9030 = t823 * t2526;
    let t9031 = t755 * t9030;
    let t9033 = t2623 * t774;
    let t9034 = t755 * t9033;
    let t9036 = t808 * t7624;
    let t9038 = t2484 * t2615;
    (t9026, t9028, t9031, t9034, t9036, t9038)
}

//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 863/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk863<F: Float>(t2526: F, t823: F, t755: F, t2623: F, t774: F, t7624: F, t808: F, t2484: F, t2615: F, t2605: F, t2489: F, t804: F, t2594: F, t158: F, t2490: F, t160: F) -> (F, F, F, F, F, F, F, F) {
    let t9030 = t823 * t2526;
    let t9031 = t755 * t9030;
    let t9033 = t2623 * t774;
    let t9034 = t755 * t9033;
    let t9036 = t808 * t7624;
    let t9038 = t2484 * t2615;
    let t9040 = t2605 * t823;
    let t9042 = t804 * t2489;
    let t9043 = t9042 * t2594;
    let t9045 = t2490 * t158;
    let t9046 = t160 * t774;
    (t9031, t9034, t9036, t9038, t9040, t9043, t9045, t9046)
}

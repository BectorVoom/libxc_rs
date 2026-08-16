//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 1202/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk1202<F: Float>(t118: F, t6347: F, t794: F, t3739: F, t12211: F, t6353: F, t213: F, t6330: F, t1307: F, t221: F, t5187: F, t5196: F) -> (F, F, F, F) {
    let t19775 = t118 * t794 * t6347;
    let t19776 = t3739 * t19775;
    let t19779 = t12211 * t6353;
    let t19781 = t213 * t6330;
    let t19783 = t221 * t19781 * t1307;
    let t19787 = t221 * t5196 * t5187;
    (t19776, t19779, t19783, t19787)
}

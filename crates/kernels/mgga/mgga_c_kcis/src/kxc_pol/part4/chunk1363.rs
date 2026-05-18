//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1363/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1363<F: Float>(t17520: F, t571: F, t3393: F, t5989: F, t12417: F, t1517: F, t1650: F, t531: F, t5867: F, t833: F, t17244: F, t509: F) -> (F, F, F, F, F) {
    let t17521 = t571 * t17520;
    let t17540 = t3393 * t5989;
    let t17543 = t1517 * t12417 * t1650;
    let t17546 = t5867 * t531;
    let t17548 = t1517 * t17546 * t833;
    let t17552 = t509 * t17244;
    (t17521, t17540, t17543, t17548, t17552)
}

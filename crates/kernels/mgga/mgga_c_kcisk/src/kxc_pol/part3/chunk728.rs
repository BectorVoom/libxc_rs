//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 728/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk728<F: Float>(t11305: F, t4609: F, t10593: F, t7012: F, t139: F, t3516: F, t41: F, t1879: F, t3521: F, t4620: F, t4600: F, t4632: F) -> (F, F, F, F, F, F, F) {
    let t11306 = t4609 * t11305;
    let t11309 = t7012 * t10593;
    let t11313 = t139 * t3516 * t41;
    let t11314 = t11313 * t1879;
    let t11316 = t3521 * t4620;
    let t11318 = t3521 * t4600;
    let t11320 = t3521 * t4632;
    (t11306, t11309, t11313, t11314, t11316, t11318, t11320)
}

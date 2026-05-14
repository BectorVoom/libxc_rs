//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 649/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk649<F: Float>(t1064: F, t4621: F, t1646: F, t331: F, t3160: F, t1071: F, t822: F, t821: F, t9: F, t7: F, t118: F) -> (F, F, F, F, F, F, F) {
    let t4866 = t1064 * t4621;
    let t4869 = t331 * t1646;
    let t4871 = t3160 * t1646;
    let t4875 = t822 * t1071;
    let t4879 = 1.0 / t9 / t821;
    let t4880 = t7 * t4879;
    let t4881 = t118 * t4880;
    (t4866, t4869, t4871, t4875, t4879, t4880, t4881)
}

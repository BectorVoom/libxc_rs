//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 832/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk832<F: Float>(t3337: F, t6693: F, t359: F, t6613: F, t376: F, t1170: F, t3217: F, t6496: F, t375: F, t1130: F, t6555: F, t355: F, t6480: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t6694 = t3337 * t6693;
    let t6696 = t359 * t6613;
    let t6697 = t376 * t6696;
    let t6698 = t1170 * t6697;
    let t6700 = t3217 * t6496;
    let t6701 = t376 * t6700;
    let t6702 = t375 * t6701;
    let t6704 = t1130 * t6555;
    let t6705 = t376 * t6704;
    let t6706 = t375 * t6705;
    let t6708 = t6480 * t355;
    (t6694, t6696, t6697, t6698, t6700, t6701, t6702, t6704, t6705, t6706, t6708)
}

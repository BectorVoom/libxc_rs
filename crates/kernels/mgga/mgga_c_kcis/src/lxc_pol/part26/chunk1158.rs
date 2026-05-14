//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1158/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1158<F: Float>(t167: F, t18079: F, t2109: F, t28752: F, t23217: F, t6176: F, t7969: F, t1615: F, t6159: F, t6284: F, t21859: F, t4160: F, t98034: F, t21007: F, t5661: F, t98530: F) -> (F, F, F, F, F) {
    let t102137 = t18079 * t28752 * t167 * t2109;
    let t102142 = t6176 * t7969 * t23217;
    let t102151 = t6159 * t28752 * t6284 * t1615;
    let t102155 = t4160 * t98034 * t21859;
    let t102158 = t5661 * t98530 * t21007;
    (t102137, t102142, t102151, t102155, t102158)
}

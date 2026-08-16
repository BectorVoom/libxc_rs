//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2985/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2985<F: Float>(t23862: F, t3172: F, t4837: F, t1041: F, t23822: F, t4866: F, t6244: F, t11710: F, t23920: F, t3091: F, t1058: F, t23961: F) -> (F, F, F, F, F) {
    let t79107 = t4837 * t3172 * t23862;
    let t79112 = t1041 * t3172 * t23822;
    let t79116 = t6244 * t4866;
    let t79139 = t3091 * t11710 * t23920;
    let t79141 = t23961 * t1058;
    (t79107, t79112, t79116, t79139, t79141)
}

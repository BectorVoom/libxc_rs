//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 229/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk229<F: Float>(t1014: F, t15: F, t183: F, t2: F, t142: F, t4: F, t151: F, t181: F, t955: F, t1011: F, t11: F, t139: F, t175: F, t197: F, t198: F, t201: F, t972: F) -> (F, F, F, F, F, F) {
    let t1015 = t1014 * t15;
    let t1016 = t183 * t2;
    let t1018 = t1016 * t4 * t142;
    let t1021 = t181 * t151;
    let t1022 = t1021 * t955;
    let t1031 = 0.619125e-2 * t1011 * t198 - 0.123825e-1 * t1015 * t1018 - 0.619125e-2 * t197 * t1022 - 0.53062222222222222221e-1 * t139 * t11 * t175 - 0.79593333333333333331e-1 * t139 * t201 * t972;
    (t1015, t1016, t1018, t1021, t1022, t1031)
}

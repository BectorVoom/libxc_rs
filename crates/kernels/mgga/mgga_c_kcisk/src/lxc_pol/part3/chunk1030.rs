//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 1030/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk1030<F: Float>(t15200: F, t15202: F, t854: F, t60: F, t12671: F, t3140: F, t979: F, t3077: F, t3141: F, t15176: F, t15179: F, t15181: F, t15183: F, t15187: F, t15191: F, t15195: F, t15198: F, t2932: F) -> (F, F, F, F) {
    let t15203 = t15200 * t15202;
    let t15206 = t854 * t854;
    let t15207 = F::new(1.0) / t15206;
    let t15208 = t60 * t15207;
    let t15211 = t12671 * t3140;
    let t15212 = t979 * t15211;
    let t15214 = t3077 * t3141;
    let t15216 = -F::new(0.29847499999999999999e-1) * t15176 - F::new(0.29847499999999999999e-1) * t15179 + F::new(0.79593333333333333331e-1) * t15181 + F::new(0.39796666666666666665e-1) * t15183 - F::new(0.59694999999999999999e-1) * t15187 + F::new(0.99491666666666666664e-2) * t15191 + F::new(0.92858888888888888885e-1) * t15195 - F::new(0.92858888888888888885e-1) * t15198 - F::new(0.223494e0) * t2932 * t15203 - F::new(0.43134342e-1) * t15208 * t15203 + F::new(0.59694999999999999999e-1) * t15212 - F::new(0.79593333333333333331e-1) * t15214;
    (t15203, t15212, t15214, t15216)
}

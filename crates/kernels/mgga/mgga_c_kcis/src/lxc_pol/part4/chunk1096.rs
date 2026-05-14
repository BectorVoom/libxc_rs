//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1096/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1096<F: Float>(t15941: F, t4148: F, t5752: F, t1394: F, t4154: F, t4153: F, t11776: F, t1947: F, t3738: F, t5781: F, t5780: F, t1494: F, t2001: F, t4129: F, t12241: F, t15909: F) -> (F, F, F, F, F, F) {
    let t15942 = 0.66327777777777777776e-2 * t15941;
    let t15943 = t5752 * t4148;
    let t15944 = t1394 * t15943;
    let t15946 = t5752 * t4154;
    let t15947 = t4153 * t15946;
    let t15949 = t11776 * t1947;
    let t15950 = t1394 * t15949;
    let t15952 = t3738 * t5781;
    let t15953 = t5780 * t15952;
    let t15955 = t1494 * t2001;
    let t15956 = t15955 * t4129;
    let t15957 = t12241 * t15956;
    let t15958 = t15909 * t15957;
    (t15942, t15944, t15947, t15950, t15953, t15958)
}

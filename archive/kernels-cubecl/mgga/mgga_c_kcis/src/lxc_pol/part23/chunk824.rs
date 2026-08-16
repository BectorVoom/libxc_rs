//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 824/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk824<F: Float>(t1394: F, t15949: F, t3738: F, t5781: F, t5780: F, t1494: F, t2001: F, t4129: F, t12241: F, t15909: F, t3734: F, t5672: F) -> (F, F, F, F, F, F) {
    let t15950 = t1394 * t15949;
    let t15952 = t3738 * t5781;
    let t15953 = t5780 * t15952;
    let t15955 = t1494 * t2001;
    let t15956 = t15955 * t4129;
    let t15957 = t12241 * t15956;
    let t15958 = t15909 * t15957;
    let t15960 = t3734 * t5672;
    (t15950, t15953, t15955, t15956, t15958, t15960)
}

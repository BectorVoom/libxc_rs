//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1236/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1236(t1394: f64, t15949: f64, t3738: f64, t5781: f64, t5780: f64, t1494: f64, t2001: f64, t4129: f64, t12241: f64, t15909: f64, t3734: f64, t5672: f64) -> (f64, f64, f64, f64) {
    let t15950 = t1394 * t15949;
    let t15952 = t3738 * t5781;
    let t15953 = t5780 * t15952;
    let t15955 = t1494 * t2001;
    let t15956 = t15955 * t4129;
    let t15957 = t12241 * t15956;
    let t15958 = t15909 * t15957;
    let t15960 = t3734 * t5672;
    (t15950, t15953, t15958, t15960)
}

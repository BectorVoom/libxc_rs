//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1155/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1155(t1586: f64, t3118: f64, t4322: f64, t1148: f64, t5294: f64, t1113: f64, t9751: f64, t1133: f64, t5248: f64, t3126: f64, t4245: f64, t9765: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t15948 = t3118 * t1586 * t4322;
    let t15952 = t5294 * t1148;
    let t15953 = t3118 * t15952;
    let t15956 = t9751 * t1113;
    let t15960 = t1133 * t5248;
    let t15964 = t3126 * t4245;
    let t15968 = t9765 * t1113;
    (t15948, t15953, t15956, t15960, t15964, t15968)
}

//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 832/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk832(t15098: f64, t352: f64, t1326: f64, t70585: f64, t69049: f64, t15241: f64, t4601: f64, t15314: f64, t56828: f64, t69057: f64, t3140: f64, t3144: f64, t9086: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t74977 = t15098 * t352;
    let t74978 = t1326 * t74977;
    let t74979 = t70585 * t74978;
    let t74981 = 0.15965655602485078085e0_f64 * t69049;
    let t74983 = 0.8980681276397856423e-1_f64 * t4601 * t15241;
    let t74984 = t56828 * t15314;
    let t74986 = 0.59590439850616975158e-4_f64 * t69057;
    let t74994 = t9086 * t3140 * t3144;
    (t74977, t74978, t74979, t74981, t74983, t74984, t74986, t74994)
}

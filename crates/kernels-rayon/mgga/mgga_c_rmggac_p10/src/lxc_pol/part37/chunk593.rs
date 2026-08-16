//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 593/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk593(t552: f64, t793: f64, t1986: f64, t3141: f64, t2060: f64, t8975: f64, t1550: f64, t8946: f64, t903: f64, t7577: f64, t8936: f64, t739: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t15234 = t793 * t552;
    let t15235 = t1986 * t15234;
    let t15236 = t3141 * t15235;
    let t15238 = t2060 * t8975;
    let t15239 = t1550 * t15238;
    let t15240 = 0.5987120850931904282e-1_f64 * t15239;
    let t15241 = t2060 * t8946;
    let t15242 = t903 * t15241;
    let t15243 = 0.8980681276397856423e-1_f64 * t15242;
    let t15244 = t7577 * t8936;
    let t15245 = t739 * t15244;
    (t15235, t15236, t15238, t15240, t15241, t15243, t15244, t15245)
}

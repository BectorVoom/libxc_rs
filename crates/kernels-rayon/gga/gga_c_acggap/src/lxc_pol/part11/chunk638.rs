//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 638/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk638(t1426: f64, t175: f64, t4822: f64, t1462: f64, t997: f64, t1173: f64, t1180: f64, t3403: f64, t397: f64, t418: f64, t4946: f64, t4949: f64, t4950: f64, t4953: f64, t4954: f64, t4957: f64, t4961: f64, t4963: f64, t4969: f64, t4971: f64, t4975: f64, t4979: f64, t4983: f64, t4989: f64, t4991: f64, t4994: f64, t4996: f64, t4999: f64) -> (f64, f64) {
    let t5003 = t1426 * t175 * t4822;
    let t5007 = 0.12004725073059526352e-1_f64 * t997 * t1462;
    let t5008 = -0.85748036236139473944e-3_f64 * t4946 + t4949 + 0.40015750243531754508e-2_f64 * t4950 - t4953 - 0.40015750243531754508e-2_f64 * t4954 - t4957 + t4961 - 0.34299214494455789578e-2_f64 * t1173 * t4963 - t4969 - 0.85748036236139473944e-2_f64 * t3403 * t4971 + 0.34299214494455789578e-2_f64 * t1173 * t4975 + 0.34299214494455789578e-2_f64 * t1173 * t4979 + 0.17149607247227894789e-2_f64 * t1180 * t4983 - t4989 + 0.17149607247227894789e-2_f64 * t1180 * t4991 + 0.17149607247227894789e-2_f64 * t4994 - 0.56688979511669985553e-2_f64 * t4996 - 0.42874018118069736972e-3_f64 * t397 * t4999 + 0.42874018118069736972e-2_f64 * t418 * t5003 - t5007;
    (t5003, t5008)
}

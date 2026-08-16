//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1161/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1161(t1188: f64, t1410: f64, t322: f64, t5853: f64, t1165: f64, t13585: f64, t5852: f64, t1173: f64, t1180: f64, t1532: f64, t1552: f64, t15930: f64, t15932: f64, t15934: f64, t15936: f64, t15938: f64, t15945: f64, t15947: f64, t1748: f64, t18834: f64, t301: f64, t3403: f64, t5606: f64, t5984: f64, t6258: f64, t839: f64) -> (f64, f64) {
    let t20935 = t1188 * t1410;
    let t20944 = t5853 * t322;
    let t20947 = t13585 * t1165 * t5852 * t20944;
    let t20959 = -0.17149607247227894789e-1_f64 * t3403 * t1165 * t1532 * t6258 * t301 - 0.85748036236139473945e-2_f64 * t3403 * t1165 * t1532 * t1748 * t839 + 0.17149607247227894789e-2_f64 * t1180 * t1165 * t1552 * t20935 - 0.51448821741683684366e-2_f64 * t1180 * t1165 * t18834 * t5984 + 0.25724410870841842184e-2_f64 * t20947 - 0.15117061203111996147e0_f64 * t15930 - 0.40015750243531754508e-2_f64 * t15932 - 0.80031500487063509016e-2_f64 * t15934 - 0.40015750243531754508e-2_f64 * t15936 - 0.16006300097412701803e-1_f64 * t15938 - 0.68598428988911579156e-2_f64 * t15945 - 0.68598428988911579156e-2_f64 * t1173 * t1165 * t15947 * t5606;
    (t20944, t20959)
}

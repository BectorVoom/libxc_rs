//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 624/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk624(t1165: f64, t1552: f64, t5606: f64, t1759: f64, t372: f64, t4313: f64, t360: f64, t1181: f64, t1891: f64, t997: f64, t1180: f64, t3454: f64, t418: f64, t4629: f64, t4635: f64, t4637: f64, t4649: f64, t4651: f64, t4653: f64, t5946: f64, t5950: f64, t5953: f64, t5956: f64, t5961: f64, t5966: f64, t5972: f64, t5975: f64, t5978: f64) -> (f64, f64, f64, f64) {
    let t5981 = t1165 * t1552 * t5606;
    let t5984 = t1759 * t372;
    let t5986 = t1165 * t4313 * t5984;
    let t5989 = t1759 * t360;
    let t5991 = t1181 * t1552 * t5989;
    let t5996 = t997 * t1891;
    let t5999 = -0.85748036236139473944e-2_f64 * t418 * t5946 + 0.85748036236139473944e-2_f64 * t418 * t5950 - 0.34299214494455789578e-2_f64 * t5953 - 0.34299214494455789578e-2_f64 * t418 * t5956 - 0.34299214494455789578e-2_f64 * t418 * t5961 - 0.34299214494455789578e-2_f64 * t418 * t5966 + 0.85748036236139473944e-3_f64 * t5972 + 0.42874018118069736972e-2_f64 * t418 * t5975 + 0.42874018118069736972e-3_f64 * t5978 + 0.85748036236139473944e-3_f64 * t1180 * t5981 - 0.25724410870841842183e-2_f64 * t1180 * t5986 + 0.17149607247227894789e-2_f64 * t1180 * t5991 + t4629 - 35.0_f64 / 108.0_f64 * t4635 - 35.0_f64 / 216.0_f64 * t4637 + t4649 + t4651 + t4653 + 0.40015750243531754507e-2_f64 * t5996 - 0.42874018118069736972e-3_f64 * t3454;
    (t5981, t5986, t5991, t5999)
}

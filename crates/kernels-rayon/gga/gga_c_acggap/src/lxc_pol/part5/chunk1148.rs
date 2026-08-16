//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1148/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1148(t1745: f64, t977: f64, t1487: f64, t6076: f64, t997: f64, t1096: f64, t1165: f64, t15626: f64, t15628: f64, t15633: f64, t15639: f64, t15653: f64, t15667: f64, t15671: f64, t165: f64, t169: f64, t171: f64, t174: f64, t1849: f64, t3462: f64, t4255: f64, t4262: f64, t5150: f64, t5862: f64) -> f64 {
    let t20650 = t977 * t1745;
    let t20652 = t1487 * t1487;
    let t20666 = t997 * t6076;
    let t20670 = -0.34299214494455789578e-2_f64 * t15626 + t4255 * t4262 * t1849 * t1096 / 2.0_f64 - 0.40015750243531754508e-2_f64 * t20650 + 0.85748036236139473944e-3_f64 * t165 * t169 * t171 * t20652 * t174 + 0.48018900292238105409e-1_f64 * t15628 + 0.10289764348336736873e-1_f64 * t15633 - 0.17149607247227894789e-2_f64 * t15639 - 0.34299214494455789578e-2_f64 * t3462 * t1165 * t5862 * t5150 + 0.85748036236139473944e-3_f64 * t15653 + 0.80031500487063509015e-2_f64 * t20666 + 7.0_f64 / 72.0_f64 * t15667 + 0.10289764348336736873e-1_f64 * t15671;
    t20670
}

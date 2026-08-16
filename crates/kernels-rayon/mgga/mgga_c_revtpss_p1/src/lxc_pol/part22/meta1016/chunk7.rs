//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3515/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3515(t13392: f64, t15787: f64, t15936: f64, t16020: f64, t16048: f64, t16052: f64, t16095: f64, t16096: f64, t16584: f64, t18941: f64, t19572: f64, t19738: f64, t19754: f64, t20066: f64, t20094: f64, t20099: f64, t2857: f64, t3092: f64, t3117: f64, t4181: f64, t42712: f64, t42716: f64, t42719: f64, t4772: f64, t4899: f64, t4902: f64, t54023: f64, t54187: f64) -> f64 {
    let t66535 = 0.45732285992607719436e-2_f64 * t16584 * t16048 * t4902 + 0.85748036236139473944e-3_f64 * t19738 * t15787 - 0.45732285992607719436e-2_f64 * t16052 * t20066 + 0.57165357490759649296e-3_f64 * t16095 * t3092 * t18941 * t16096 + 0.11433071498151929859e-2_f64 * t16095 * t3092 * t4772 * t2857 * t4181 - 0.17149607247227894789e-2_f64 * t16095 * t3092 * t20099 * t15936 + 0.57165357490759649296e-3_f64 * t16095 * t3092 * t20094 * t13392 + 0.13719685797782315831e-1_f64 * t54023 * t19754 - 0.21437009059034868486e-3_f64 * t4899 * t3117 * t19572 * t16020 + 0.28582678745379824648e-3_f64 * t54187 + t42712 / 243.0_f64 + 5.0_f64 / 1944.0_f64 * t42716 + t42719 / 648.0_f64;
    t66535
}

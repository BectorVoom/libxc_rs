//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3518/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3518(t15905: f64, t56017: f64, t55899: f64, t11703: f64, t11859: f64, t15606: f64, t15609: f64, t15908: f64, t15910: f64, t16020: f64, t16025: f64, t16067: f64, t16084: f64, t16095: f64, t16096: f64, t18936: f64, t19450: f64, t19501: f64, t19572: f64, t19758: f64, t19954: f64, t3117: f64, t3241: f64, t42675: f64, t43044: f64, t4891: f64, t4902: f64, t53669: f64, t54314: f64, t54324: f64, t54570: f64, t55985: f64, t64891: f64) -> f64 {
    let t66621 = t56017 * t15905;
    let t66624 = t55899 * t15905;
    let t66631 = -4.0_f64 / 243.0_f64 * t54314 - 4.0_f64 / 81.0_f64 * t3241 * t19954 + 0.85748036236139473944e-3_f64 * t54570 * t15606 - 0.85748036236139473944e-3_f64 * t11859 * t3117 * t19572 * t15609 + 0.21437009059034868486e-3_f64 * t16067 * t3117 * t19450 * t16020 - 0.47637797908966374414e-3_f64 * t16095 * t11703 * t18936 * t16096 + 0.30011812682648815881e-2_f64 * t53669 * t3117 * t64891 * t15908 - 0.42874018118069736972e-3_f64 * t43044 * t3117 * t19501 * t16025 - 0.22866142996303859718e-2_f64 * t42675 * t19758 + 0.25724410870841842183e-2_f64 * t66621 * t16084 - 0.25724410870841842183e-2_f64 * t66624 * t15910 + 0.30488190661738479624e-2_f64 * t54324 - 0.85748036236139473944e-3_f64 * t55985 * t4891 * t4902;
    t66631
}

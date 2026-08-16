//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1171/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1171(t1035: f64, t1651: f64, t1646: f64, t1043: f64, t1089: f64, t1096: f64, t120297: f64, t120321: f64, t120322: f64, t120448: f64, t120513: f64, t120636: f64, t120664: f64, t120702: f64, t126442: f64, t1976: f64, t1982: f64, t27543: f64, t3153: f64, t31891: f64, t31892: f64, t31897: f64, t31903: f64, t31959: f64, t32015: f64, t33787: f64, t33791: f64, t33796: f64, t33803: f64, t33817: f64, t4742: f64, t4763: f64, t4998: f64, t7135: f64, t7145: f64, t7837: f64, t8507: f64, t988: f64) -> f64 {
    let t126953 = t1035 * t1651;
    let t126965 = t1035 * t1646;
    let t126995 = 0.11423947533020470523e1_f64 * t31891 * t31892 * t1976 * t27543 + 0.17347256376410398924e1_f64 * t120664 * t126953 * t1043 * t1089 + 0.34694512752820797848e1_f64 * t120448 * t7145 * t126442 + 0.8673628188205199462e0_f64 * t120636 * t33787 * t3153 * t4998 - 0.17347256376410398924e1_f64 * t120513 * t126965 * t1043 * t1089 + 0.51407763898592117355e1_f64 * t31903 * t31959 * t33796 * t1096 - 0.51407763898592117355e1_f64 * t31897 * t31959 * t33803 * t988 + 0.56468933516960933998e-3_f64 * t120321 * t32015 * t120322 * t4763 - 0.5578099381357651623e-3_f64 * t120297 * t33817 + 0.17135921299530705785e1_f64 * t31897 * t31892 * t8507 * t4742 - 0.17347256376410398924e1_f64 * t1982 * t7135 * t7837 + 0.3427184259906141157e1_f64 * t120702 * t31892 * t33791 * t988;
    t126995
}

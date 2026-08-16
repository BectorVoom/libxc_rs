//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3861/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3861(t125: f64, t21969: f64, t1399: f64, t6883: f64, t9816: f64, t9818: f64, t13999: f64, t22271: f64, t48919: f64, t6869: f64, t13716: f64, t13944: f64, t1872: f64, t22096: f64, t3889: f64, t3934: f64, t3936: f64, t3944: f64, t48508: f64, t48510: f64, t48595: f64, t543: f64, t5674: f64, t6849: f64, t800: f64, t9748: f64) -> f64 {
    let t74177 = t125 * t21969;
    let t74184 = t9816 * t9818 * t6883 * t1399;
    let t74186 = t13999 * t22271;
    let t74206 = t9816 * t9818 * t48919 * t6869;
    let t74215 = 0.17149607247227894789e-2_f64 * t3934 * t3936 * t74177 * t1399 + 0.10164000561857065645e-3_f64 * t74184 - 0.40015750243531754508e-2_f64 * t74186 - t9748 * t800 * t6849 * t3889 / 4.0_f64 + t3944 * t800 * t1872 * t13716 / 8.0_f64 + 0.17149607247227894789e-2_f64 * t3934 * t3936 * t48595 * t6869 + 0.34299214494455789578e-2_f64 * t3934 * t3936 * t13944 * t22096 + 0.2032800112371413129e-3_f64 * t74206 + 0.17149607247227894789e-2_f64 * t3934 * t3936 * t5674 * t543 * t13716 - 0.10841600599314203355e-1_f64 * t48508 - 0.80031500487063509016e-1_f64 * t48510;
    t74215
}

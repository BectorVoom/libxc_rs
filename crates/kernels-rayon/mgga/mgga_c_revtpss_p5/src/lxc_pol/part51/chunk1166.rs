//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1166/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1166(t1028: f64, t1032: f64, t1042: f64, t120263: f64, t120479: f64, t126600: f64, t126749: f64, t126765: f64, t126770: f64, t126774: f64, t126779: f64, t247: f64, t3046: f64, t3116: f64, t31891: f64, t31892: f64, t31899: f64, t31903: f64, t31913: f64, t31959: f64, t32006: f64, t32014: f64, t32015: f64, t33751: f64, t385: f64, t4742: f64, t4772: f64, t4872: f64, t4946: f64, t5015: f64, t8501: f64, t8507: f64) -> f64 {
    let t126786 = -0.17135921299530705785e1_f64 * t31903 * t31892 * t8507 * t4772 + 0.17135921299530705785e1_f64 * t126749 * t31899 + 0.24791552806034007214e-3_f64 * t120263 * t1042 * t4872 * t126600 + 0.56468933516960933998e-3_f64 * t3046 * t1032 * t8501 * t33751 + 0.56468933516960933998e-3_f64 * t31913 * t247 * t3116 * t385 * t4742 + 0.56468933516960933998e-3_f64 * t32014 * t32015 * t126765 * t4946 - 0.5578099381357651623e-3_f64 * t126770 * t32006 + 0.5578099381357651623e-3_f64 * t126774 * t1028 + 0.24791552806034007213e-3_f64 * t126779 - 0.12395776403017003607e-3_f64 * t120479 - 0.17135921299530705785e1_f64 * t31891 * t31959 * t8507 * t5015;
    t126786
}

//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1161/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1161(t1078: f64, t905: f64, t1096: f64, t1469: f64, t31991: f64, t33800: f64, t127: f64, t32003: f64, t33815: f64, t371: f64, t1045: f64, t120191: f64, t120218: f64, t120329: f64, t120362: f64, t120473: f64, t3092: f64, t3116: f64, t3117: f64, t31975: f64, t31993: f64, t31994: f64, t32010: f64, t32014: f64, t33817: f64, t372: f64, t373: f64, t385: f64, t4742: f64, t4763: f64, t4838: f64, t4854: f64, t4866: f64, t4940: f64, t7160: f64, t99970: f64) -> (f64, f64) {
    let t126599 = t1078 * t905;
    let t126600 = t1469 * t1096;
    let t126620 = t33800 * t31991;
    let t126636 = t32003 * t371 * t127 * t33815;
    let t126640 = -0.18822977838986977999e-3_f64 * t32014 * t3092 * t126599 * t126600 - 0.34694512752820797848e1_f64 * t120362 * t7160 * t4940 + 0.28234466758480466999e-3_f64 * t31975 * t3117 * t385 * t4866 * t1045 + 0.16734298144072954869e-2_f64 * t120218 * t31993 * t3116 * t99970 - 0.22312397525430606492e-2_f64 * t120473 * t31993 * t4838 - 0.12395776403017003607e-3_f64 * t126620 * t31994 - 0.11156198762715303246e-2_f64 * t120329 * t31993 * t3116 * t4763 - 0.5578099381357651623e-3_f64 * t32003 * t371 * t372 * t373 * t4742 + 0.29749863367240808656e-2_f64 * t120191 * t33817 - 0.3718732920905101082e-3_f64 * t126636 + 0.5578099381357651623e-3_f64 * t32010 * t4854;
    (t126600, t126640)
}

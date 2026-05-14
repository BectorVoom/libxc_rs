//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1017/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1017<F: Float>(t1078: F, t905: F, t1096: F, t1469: F, t31991: F, t33800: F, t127: F, t32003: F, t33815: F, t371: F, t1045: F, t120191: F, t120218: F, t120329: F, t120362: F, t120473: F, t3092: F, t3116: F, t3117: F, t31975: F, t31993: F, t31994: F, t32010: F, t32014: F, t33817: F, t372: F, t373: F, t385: F, t4742: F, t4763: F, t4838: F, t4854: F, t4866: F, t4940: F, t7160: F, t99970: F) -> (F, F) {
    let t126599 = t1078 * t905;
    let t126600 = t1469 * t1096;
    let t126620 = t33800 * t31991;
    let t126636 = t32003 * t371 * t127 * t33815;
    let t126640 = -0.18822977838986977999e-3 * t32014 * t3092 * t126599 * t126600 - 0.34694512752820797848e1 * t120362 * t7160 * t4940 + 0.28234466758480466999e-3 * t31975 * t3117 * t385 * t4866 * t1045 + 0.16734298144072954869e-2 * t120218 * t31993 * t3116 * t99970 - 0.22312397525430606492e-2 * t120473 * t31993 * t4838 - 0.12395776403017003607e-3 * t126620 * t31994 - 0.11156198762715303246e-2 * t120329 * t31993 * t3116 * t4763 - 0.5578099381357651623e-3 * t32003 * t371 * t372 * t373 * t4742 + 0.29749863367240808656e-2 * t120191 * t33817 - 0.3718732920905101082e-3 * t126636 + 0.5578099381357651623e-3 * t32010 * t4854;
    (t126600, t126640)
}

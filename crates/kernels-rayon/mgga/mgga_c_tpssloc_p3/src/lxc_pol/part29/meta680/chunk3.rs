//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2289/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2289(t24574: f64, t27779: f64, t8015: f64, t85660: f64, t27826: f64, t11606: f64, t11925: f64, t1238: f64, t12652: f64, t15771: f64, t15789: f64, t2121: f64, t2155: f64, t225: f64, t24564: f64, t24591: f64, t24601: f64, t27406: f64, t27549: f64, t27774: f64, t27784: f64, t27785: f64, t27792: f64, t3598: f64, t3599: f64, t3600: f64, t462: f64, t497: f64, t5088: f64, t53658: f64, t7391: f64, t8087: f64, t8088: f64, t86426: f64, t94395: f64) -> f64 {
    let t94700 = 0.18277045187202515961e-2_f64 * t24574 * t27779;
    let t94701 = t85660 * t8015;
    let t94710 = 0.54831135561607547884e-2_f64 * t24574 * t27826;
    let t94734 = -t94700 + 0.18277045187202515961e-2_f64 * t94701 + 0.54831135561607547884e-2_f64 * t86426 - 12.0_f64 * t27784 * t27785 * t15789 + 0.21932454224643019153e-1_f64 * t27406 * t24564 - t94710 - t53658 * t2155 + 0.82246703342411321825e-2_f64 * t2121 * t462 * t15771 * t225 * t497 + 4.0_f64 * t1238 * t3598 * t7391 * t5088 - 6.0_f64 * t1238 * t11606 * t8087 * t3599 + 2.0_f64 * t27792 * t3600 - t11925 * t8088 - 0.14621636149762012769e-1_f64 * t94395 * t24591 + 0.73108180748810063846e-2_f64 * t27549 * t24601 * t27774 * t12652;
    t94734
}

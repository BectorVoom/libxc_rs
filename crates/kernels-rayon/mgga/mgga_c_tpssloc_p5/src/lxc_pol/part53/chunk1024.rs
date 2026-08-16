//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 1024/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk1024(t1484: f64, t7109: f64, t7844: f64, t868: f64, t193: f64, t200: f64, t8747: f64, t8753: f64, t116473: f64, t116476: f64, t116481: f64, t118417: f64, t118440: f64, t123715: f64, t123719: f64, t123733: f64, t1408: f64, t1877: f64, t22960: f64, t24191: f64, t25: f64, t25021: f64, t2522: f64, t25366: f64, t25373: f64, t25375: f64, t25377: f64, t25381: f64, t25385: f64, t26756: f64, t32030: f64, t32034: f64, t32047: f64, t33991: f64, t606: f64, t6542: f64, t6671: f64, t7114: f64, t7475: f64, t7545: f64, t8744: f64, t8748: f64) -> (f64, f64, f64, f64, f64) {
    let t123745 = t1484 * t7109;
    let t123752 = t7844 * t868;
    let t123757 = t193 * t200 * t8747;
    let t123764 = t193 * t8753;
    let t123766 = t1877 * t32047 * t25377 - t1877 * t32034 * t25381 / 2.0_f64 + t1877 * t123715 * t25 / 2.0_f64 - t1877 * t123719 * t6671 / 2.0_f64 + 3.0_f64 / 2.0_f64 * t2522 * t33991 * t6542 + 3.0_f64 / 2.0_f64 * t2522 * t32030 * t7475 - t1877 * t7114 * t606 * t7844 - t123733 + 3.0_f64 / 2.0_f64 * t2522 * t8744 * t25385 - 3.0_f64 / 2.0_f64 * t2522 * t8748 * t25385 - t1877 * t116476 * t7545 / 2.0_f64 - 3.0_f64 / 2.0_f64 * t116473 * t25366 - 3.0_f64 * t24191 * t22960 * t123745 + t1877 * t32030 * t1408 / 2.0_f64 + 2.0_f64 * t26756 * t25373 * t123752 - 3.0_f64 * t123757 * t118440 - 3.0_f64 / 2.0_f64 * t116473 * t25021 + 3.0_f64 * t116481 * t118417 + t123764 * t25375;
    (t123745, t123752, t123757, t123764, t123766)
}

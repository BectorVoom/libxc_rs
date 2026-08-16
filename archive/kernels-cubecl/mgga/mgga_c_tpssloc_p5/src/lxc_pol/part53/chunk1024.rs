//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 1024/1059 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk1024<F: Float>(t1484: F, t7109: F, t7844: F, t868: F, t193: F, t200: F, t8747: F, t8753: F, t116473: F, t116476: F, t116481: F, t118417: F, t118440: F, t123715: F, t123719: F, t123733: F, t1408: F, t1877: F, t22960: F, t24191: F, t25: F, t25021: F, t2522: F, t25366: F, t25373: F, t25375: F, t25377: F, t25381: F, t25385: F, t26756: F, t32030: F, t32034: F, t32047: F, t33991: F, t606: F, t6542: F, t6671: F, t7114: F, t7475: F, t7545: F, t8744: F, t8748: F) -> (F, F, F, F, F) {
    let t123745 = t1484 * t7109;
    let t123752 = t7844 * t868;
    let t123757 = t193 * t200 * t8747;
    let t123764 = t193 * t8753;
    let t123766 = t1877 * t32047 * t25377 - t1877 * t32034 * t25381 / F::cast_from(2.0_f64) + t1877 * t123715 * t25 / F::cast_from(2.0_f64) - t1877 * t123719 * t6671 / F::cast_from(2.0_f64) + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t2522 * t33991 * t6542 + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t2522 * t32030 * t7475 - t1877 * t7114 * t606 * t7844 - t123733 + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t2522 * t8744 * t25385 - F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t2522 * t8748 * t25385 - t1877 * t116476 * t7545 / F::cast_from(2.0_f64) - F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t116473 * t25366 - F::cast_from(3.0_f64) * t24191 * t22960 * t123745 + t1877 * t32030 * t1408 / F::cast_from(2.0_f64) + F::cast_from(2.0_f64) * t26756 * t25373 * t123752 - F::cast_from(3.0_f64) * t123757 * t118440 - F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t116473 * t25021 + F::cast_from(3.0_f64) * t116481 * t118417 + t123764 * t25375;
    (t123745, t123752, t123757, t123764, t123766)
}

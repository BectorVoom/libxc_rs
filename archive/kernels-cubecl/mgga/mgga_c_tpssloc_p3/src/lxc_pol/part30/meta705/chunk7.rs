//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2315/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2315<F: Float>(t23384: F, t28610: F, t28557: F, t6743: F, t1058: F, t1060: F, t11034: F, t1409: F, t1539: F, t23633: F, t23635: F, t23685: F, t25497: F, t28601: F, t28642: F, t28674: F, t3180: F, t3200: F, t4649: F, t4669: F, t4684: F, t5681: F, t5866: F, t6687: F, t6768: F, t6784: F, t6797: F, t6800: F, t6801: F, t82830: F, t89235: F, t89449: F) -> F {
    let t100399 = t23384 * t28610;
    let t100417 = t28557 * t6743;
    let t100430 = F::cast_from(0.36554090374405031923e-2_f64) * t89449 + F::cast_from(0.18277045187202515961e-2_f64) * t100399 - F::cast_from(0.54831135561607547884e-2_f64) * t6687 * t6784 * t23685 * t5681 - F::cast_from(2.0_f64) * t3200 * t28601 * t4684 + F::cast_from(2.0_f64) * t4669 * t25497 + t1058 * t6768 * t5866 * t1060 + F::cast_from(0.54831135561607547884e-2_f64) * t6687 * t6784 * t89235 * t1539 - F::cast_from(0.82246703342411321825e-2_f64) * t6797 * t100417 * t6801 + t3180 * t28642 + F::cast_from(0.54831135561607547884e-2_f64) * t23633 * t23635 * t1409 * t4649 * t6800 + F::cast_from(2.0_f64) * t11034 * t28674 + F::cast_from(0.48738787165873375896e-2_f64) * t82830;
    t100430
}

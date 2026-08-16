//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2315/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2315(t23384: f64, t28610: f64, t28557: f64, t6743: f64, t1058: f64, t1060: f64, t11034: f64, t1409: f64, t1539: f64, t23633: f64, t23635: f64, t23685: f64, t25497: f64, t28601: f64, t28642: f64, t28674: f64, t3180: f64, t3200: f64, t4649: f64, t4669: f64, t4684: f64, t5681: f64, t5866: f64, t6687: f64, t6768: f64, t6784: f64, t6797: f64, t6800: f64, t6801: f64, t82830: f64, t89235: f64, t89449: f64) -> f64 {
    let t100399 = t23384 * t28610;
    let t100417 = t28557 * t6743;
    let t100430 = 0.36554090374405031923e-2_f64 * t89449 + 0.18277045187202515961e-2_f64 * t100399 - 0.54831135561607547884e-2_f64 * t6687 * t6784 * t23685 * t5681 - 2.0_f64 * t3200 * t28601 * t4684 + 2.0_f64 * t4669 * t25497 + t1058 * t6768 * t5866 * t1060 + 0.54831135561607547884e-2_f64 * t6687 * t6784 * t89235 * t1539 - 0.82246703342411321825e-2_f64 * t6797 * t100417 * t6801 + t3180 * t28642 + 0.54831135561607547884e-2_f64 * t23633 * t23635 * t1409 * t4649 * t6800 + 2.0_f64 * t11034 * t28674 + 0.48738787165873375896e-2_f64 * t82830;
    t100430
}

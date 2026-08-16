//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 842/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk842(t12868: f64, t5907: f64, t3831: f64, t458: f64, t1364: f64, t3593: f64, t457: f64, t1430: f64, t3517: f64, t1435: f64, t1202: f64, t3721: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t12869 = t5907 * t12868;
    let t12872 = t458 * t3831;
    let t12873 = t3593 * t1364;
    let t12874 = t12872 * t12873;
    let t12875 = t457 * t12874;
    let t12878 = t3517 * t1430;
    let t12880 = t3517 * t1435;
    let t12884 = 1.0_f64 / t3721 / t1202;
    (t12869, t12873, t12874, t12875, t12878, t12880, t12884)
}

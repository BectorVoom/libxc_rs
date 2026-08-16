//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 228/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk228(t886: f64, t917: f64, t307: f64, t302: f64, t880: f64, t906: f64, t897: f64, t902: f64, t910: f64, t310: f64, t324: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t919 = -t917 - 0.17123333333333333333e-1_f64 * t886;
    let t922 = t307 * t307;
    let t923 = 1.0_f64 / t922;
    let t924 = t302 * t923;
    let t926 = 0.516475e0_f64 * t880;
    let t929 = 0.104195e0_f64 * t906;
    let t931 = 0.3529725e1_f64 * t897 - t926 - 0.516475e0_f64 * t886 + 0.6311625e0_f64 * t902 - t929 - 0.104195e0_f64 * t910;
    let t932 = 1.0_f64 / t310;
    let t933 = t931 * t932;
    let t936 = 0.92708333333333333333e-2_f64 * t880;
    let t938 = -t936 - 0.92708333333333333333e-2_f64 * t886;
    let t939 = t938 * t324;
    (t919, t922, t923, t924, t926, t929, t931, t932, t933, t936, t938, t939)
}

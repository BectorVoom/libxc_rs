//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 648/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk648(t5248: f64, t5249: f64, t7715: f64, t1919: f64, t2063: f64, t7389: f64, t5259: f64, t1920: f64, t7718: f64, t673: f64, t8662: f64, t140: f64, t1470: f64, t2517: f64, t2521: f64, t2543: f64, t479: f64, t5231: f64, t5242: f64, t709: f64, t725: f64, t7368: f64, t7387: f64, t8915: f64, t8919: f64, t8923: f64, t8927: f64, t8931: f64, t8975: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8995 = t5248 * t5249 * t7715;
    let t8999 = t1919 * t7389 * t2063;
    let t9003 = t1919 * t5259 * t7715;
    let t9007 = t1919 * t1920 * t7718;
    let t9010 = t673 * t8662;
    let t9014 = 0.619125e-2_f64 * t8975 * t709 + 0.1857375e-1_f64 * t2543 * t2517 - 0.123825e-1_f64 * t2543 * t2521 + 0.46434375e-2_f64 * t725 * t8915 - 0.1857375e-1_f64 * t5231 * t8919 + 0.9286875e-2_f64 * t725 * t8923 + 0.123825e-1_f64 * t725 * t8927 - 0.619125e-2_f64 * t725 * t8931 + t5242 - 0.35374814814814814814e-1_f64 * t7368 - 0.53062222222222222222e-1_f64 * t7387 - 0.44218518518518518518e-1_f64 * t1470 * t8995 - 0.53062222222222222222e-1_f64 * t1470 * t8999 + 0.53062222222222222222e-1_f64 * t1470 * t9003 - 0.26531111111111111111e-1_f64 * t1470 * t9007 - 0.39796666666666666666e-1_f64 * t140 * t479 * t9010;
    (t8995, t8999, t9003, t9007, t9010, t9014)
}

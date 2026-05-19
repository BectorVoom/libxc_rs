//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 648/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk648<F: Float>(t5248: F, t5249: F, t7715: F, t1919: F, t2063: F, t7389: F, t5259: F, t1920: F, t7718: F, t673: F, t8662: F, t140: F, t1470: F, t2517: F, t2521: F, t2543: F, t479: F, t5231: F, t5242: F, t709: F, t725: F, t7368: F, t7387: F, t8915: F, t8919: F, t8923: F, t8927: F, t8931: F, t8975: F) -> (F, F, F, F, F, F) {
    let t8995 = t5248 * t5249 * t7715;
    let t8999 = t1919 * t7389 * t2063;
    let t9003 = t1919 * t5259 * t7715;
    let t9007 = t1919 * t1920 * t7718;
    let t9010 = t673 * t8662;
    let t9014 = F::new(0.619125e-2) * t8975 * t709 + F::new(0.1857375e-1) * t2543 * t2517 - F::new(0.123825e-1) * t2543 * t2521 + F::new(0.46434375e-2) * t725 * t8915 - F::new(0.1857375e-1) * t5231 * t8919 + F::new(0.9286875e-2) * t725 * t8923 + F::new(0.123825e-1) * t725 * t8927 - F::new(0.619125e-2) * t725 * t8931 + t5242 - F::cast_from(0.35374814814814814814e-1_f64) * t7368 - F::cast_from(0.53062222222222222222e-1_f64) * t7387 - F::cast_from(0.44218518518518518518e-1_f64) * t1470 * t8995 - F::cast_from(0.53062222222222222222e-1_f64) * t1470 * t8999 + F::cast_from(0.53062222222222222222e-1_f64) * t1470 * t9003 - F::cast_from(0.26531111111111111111e-1_f64) * t1470 * t9007 - F::cast_from(0.39796666666666666666e-1_f64) * t140 * t479 * t9010;
    (t8995, t8999, t9003, t9007, t9010, t9014)
}

//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 439/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk439<F: Float>(t1078: F, t3313: F, t2864: F, t2867: F, t2869: F, t2873: F, t2875: F, t2877: F, t1070: F, t242: F, t250: F, t142: F, t841: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t3314 = t3313 * t1078;
    let t3323 = -F::new(0.78438333333333333333e0) * t2864 + F::new(0.15687666666666666667e1) * t2867 + F::new(0.68863333333333333333e0) * t2869 + F::new(0.14025833333333333333e0) * t2873 + F::new(0.28051666666666666667e0) * t2875 + F::new(0.17365833333333333333e0) * t2877;
    let t3324 = t3323 * t1078;
    let t3327 = t1070 * t1070;
    let t3328 = F::new(1.0) / t3327;
    let t3329 = t242 * t3328;
    let t3330 = t250 * t250;
    let t3331 = F::new(1.0) / t3330;
    let t3332 = t3313 * t3331;
    let t3338 = t142 * t841;
    (t3314, t3323, t3324, t3327, t3328, t3329, t3330, t3331, t3332, t3338)
}

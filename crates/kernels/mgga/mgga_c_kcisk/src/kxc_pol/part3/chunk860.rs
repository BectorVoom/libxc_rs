//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 860/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk860<F: Float>(t14160: F, t14162: F, t14167: F, t14171: F, t14173: F, t14177: F, t14179: F, t14181: F, t14185: F, t14190: F, t14193: F, t14195: F, t1299: F, t3795: F, t3799: F, t3482: F) -> (F, F) {
    let t14197 = -0.16581944444444444444e-2 * t14160 + 0.48640370370370370369e-1 * t14162 - 0.11349419753086419753e0 * t14167 + 0.2653111111111111111e-1 * t14171 - 0.2653111111111111111e-1 * t14173 - 0.49745833333333333332e-2 * t14177 - 0.66327777777777777776e-2 * t14179 - 0.49745833333333333332e-2 * t14181 + 0.19898333333333333333e-1 * t14185 + 0.1492375e-1 * t14190 + 0.39796666666666666665e-1 * t14193 + 0.66327777777777777776e-2 * t14195;
    let t14199 = t3795 * t1299;
    let t14200 = t14199 * t3799;
    let t14201 = t3482 * t14200;
    (t14197, t14201)
}

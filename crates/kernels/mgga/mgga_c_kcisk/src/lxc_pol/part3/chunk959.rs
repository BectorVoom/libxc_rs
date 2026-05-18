//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 959/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk959<F: Float>(t1415: F, t14183: F, t1411: F, t1404: F, t3783: F, t3787: F, t3508: F, t3791: F, t3513: F, t3739: F, t14160: F, t14162: F, t14167: F, t14171: F, t14173: F, t14177: F, t14179: F, t14181: F, sigma0: F) -> (F, F, F, F, F, F) {
    let t14184 = t1415 * t14183;
    let t14185 = t1411 * t14184;
    let t14187 = t1404 * t3783;
    let t14188 = t14187 * sigma0;
    let t14189 = t14188 * t3787;
    let t14190 = t1411 * t14189;
    let t14192 = t3508 * t3791;
    let t14193 = t1411 * t14192;
    let t14195 = t3739 * t3513;
    let t14197 = -F::new(0.16581944444444444444e-2) * t14160 + F::new(0.48640370370370370369e-1) * t14162 - F::new(0.11349419753086419753e0) * t14167 + F::new(0.2653111111111111111e-1) * t14171 - F::new(0.2653111111111111111e-1) * t14173 - F::new(0.49745833333333333332e-2) * t14177 - F::new(0.66327777777777777776e-2) * t14179 - F::new(0.49745833333333333332e-2) * t14181 + F::new(0.19898333333333333333e-1) * t14185 + F::new(0.1492375e-1) * t14190 + F::new(0.39796666666666666665e-1) * t14193 + F::new(0.66327777777777777776e-2) * t14195;
    (t14185, t14187, t14190, t14193, t14195, t14197)
}

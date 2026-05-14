//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 918/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk918<F: Float>(t1421: F, t19163: F, t19235: F, t26632: F, t26692: F, t31034: F, t31038: F, t31042: F, t31046: F, t31050: F, t31054: F, t31057: F, t2083: F, t3539: F, t7744: F, t2075: F, t7764: F) -> (F, F, F) {
    let t31060 = 0.39422578e-2 * t26632 - 0.98556445e-3 * t19163 - 0.26281718666666666667e-2 * t26692 + 0.65704296666666666665e-3 * t19235 - 0.65704296666666666666e-2 * t1421 * t31034 + 0.39422577999999999999e-2 * t1421 * t31038 - 0.4435040025e-2 * t1421 * t31042 - 0.4435040025e-2 * t1421 * t31046 + 0.49278222499999999999e-2 * t1421 * t31050 - 0.32852148333333333333e-2 * t1421 * t31054 + 0.32852148333333333333e-2 * t1421 * t31057;
    let t31063 = t3539 * t7744 * t2083;
    let t31066 = t2075 * t7764;
    (t31060, t31063, t31066)
}

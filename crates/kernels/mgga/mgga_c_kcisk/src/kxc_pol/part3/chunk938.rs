//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 938/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk938<F: Float>(t1390: F, t301: F, t1310: F, t12829: F, t403: F, t12830: F, t1311: F, t164: F, t1314: F, t1309: F, t3966: F, t3974: F) -> (F, F, F, F, F) {
    let t13893 = F::new(1.0) / t301 / t1390;
    let t13894 = t1310 * t13893;
    let t13895 = t403 * t12829;
    let t13896 = t13895 * t12830;
    let t13897 = t13894 * t13896;
    let t13900 = t164 * t1311;
    let t13901 = t13900 * t1314;
    let t13902 = t1309 * t13901;
    let t13906 = t3966 * t3974;
    (t13894, t13897, t13900, t13902, t13906)
}

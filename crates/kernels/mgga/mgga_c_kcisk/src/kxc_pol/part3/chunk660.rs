//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 660/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk660<F: Float>(t10933: F, t3118: F, t353: F, t579: F, t609: F, t615: F, t1709: F, t4865: F, t10937: F, t10941: F, t10944: F, t10947: F, t10951: F, t10954: F, t10957: F, t10960: F, t10963: F, t10966: F) -> (F, F, F, F, F, F) {
    let t11030 = 0.93011851851851851854e0 * t10933;
    let t11032 = t353 * t3118 * t579;
    let t11033 = 0.73028148148148148147e0 * t11032;
    let t11036 = 1.0 / t609 / t615 / 8.0;
    let t11037 = t4865 * t1709;
    let t11038 = t11036 * t11037;
    let t11040 = 28.0 / 27.0 * t10933;
    let t11051 = -t11040 - 4.0 / 9.0 * t10937 + 2.0 / 9.0 * t10941 - 2.0 / 3.0 * t10944 + t10947 / 3.0 - 10.0 / 27.0 * t10951 + 4.0 / 3.0 * t10954 - 2.0 / 3.0 * t10957 - 2.0 * t10960 + 2.0 * t10963 - t10966 / 3.0;
    (t11030, t11032, t11033, t11037, t11038, t11051)
}

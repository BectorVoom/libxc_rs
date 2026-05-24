//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 711/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk711<F: Float>(t10933: F, t3118: F, t353: F, t579: F, t609: F, t615: F, t1709: F, t4865: F, t10937: F, t10941: F, t10944: F, t10947: F, t10951: F, t10954: F, t10957: F, t10960: F, t10963: F, t10966: F) -> (F, F, F, F, F, F) {
    let t11030 = F::cast_from(0.93011851851851851854e0_f64) * t10933;
    let t11032 = t353 * t3118 * t579;
    let t11033 = F::cast_from(0.73028148148148148147e0_f64) * t11032;
    let t11036 = F::new(1.0) / t609 / t615 / F::new(8.0);
    let t11037 = t4865 * t1709;
    let t11038 = t11036 * t11037;
    let t11040 = F::new(28.0) / F::new(27.0) * t10933;
    let t11051 = -t11040 - F::new(4.0) / F::new(9.0) * t10937 + F::new(2.0) / F::new(9.0) * t10941 - F::new(2.0) / F::new(3.0) * t10944 + t10947 / F::new(3.0) - F::new(10.0) / F::new(27.0) * t10951 + F::new(4.0) / F::new(3.0) * t10954 - F::new(2.0) / F::new(3.0) * t10957 - F::new(2.0) * t10960 + F::new(2.0) * t10963 - t10966 / F::new(3.0);
    (t11030, t11032, t11033, t11037, t11038, t11051)
}

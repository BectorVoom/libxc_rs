//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 678/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk678<F: Float>(t4907: F, t617: F, t608: F, t163: F, t1774: F, t24: F, t5005: F, t10933: F, t3118: F, t353: F, t579: F, t609: F, t615: F) -> (F, F, F, F, F, F, F) {
    let t10982 = F::cast_from(1.0_f64) / t4907 / t617;
    let t10983 = t608 * t10982;
    let t10999 = t163 * t1774;
    let t11003 = t24 * t5005;
    let t11030 = F::cast_from(0.93011851851851851854e0_f64) * t10933;
    let t11032 = t353 * t3118 * t579;
    let t11033 = F::cast_from(0.73028148148148148147e0_f64) * t11032;
    let t11036 = F::cast_from(1.0_f64) / t609 / t615 / F::cast_from(8.0_f64);
    (t10983, t10999, t11003, t11030, t11032, t11033, t11036)
}

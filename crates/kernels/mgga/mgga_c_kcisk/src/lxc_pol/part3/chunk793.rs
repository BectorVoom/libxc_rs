//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 793/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk793<F: Float>(t12246: F, t782: F, t2009: F, t5465: F, t2005: F, t5477: F, t2019: F, t657: F, t2023: F, t5509: F, t1586: F, t163: F, t397: F) -> (F, F, F, F, F) {
    let t12248 = F::new(0.9994882620098509563e-2) * t782 * t12246;
    let t12249 = t5465 * t2009;
    let t12251 = t2005 * t5477;
    let t12253 = t2019 * t2019;
    let t12254 = F::new(1.0) / t12253;
    let t12255 = t657 * t12254;
    let t12256 = t5509 * t2023;
    let t12257 = t12255 * t12256;
    let t12258 = t1586 * t12257;
    let t12261 = t397 * t163;
    (t12248, t12249, t12251, t12258, t12261)
}

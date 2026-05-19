//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 689/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk689<F: Float>(t10487: F, t786: F, t1849: F, t2020: F, t10791: F, t397: F, t782: F, t2019: F, t657: F, t163: F, t4597: F, t2040: F) -> (F, F, F, F, F, F, F) {
    let t12198 = t786 * t10487;
    let t12234 = t2020 * t1849;
    let t12246 = t397 * t10791 * t786;
    let t12248 = F::cast_from(0.9994882620098509563e-2_f64) * t782 * t12246;
    let t12253 = t2019 * t2019;
    let t12254 = F::new(1.0) / t12253;
    let t12255 = t657 * t12254;
    let t12261 = t397 * t163;
    let t12271 = t2020 * t4597;
    let t12350 = t2040 * t2040;
    (t12198, t12234, t12248, t12255, t12261, t12271, t12350)
}

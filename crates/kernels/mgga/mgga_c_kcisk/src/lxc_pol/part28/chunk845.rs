//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 845/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk845<F: Float>(t10463: F, t786: F, t10879: F, t2015: F, t2013: F, t10487: F, t1849: F, t2020: F, t10791: F, t397: F, t782: F, t2005: F, t5477: F, t2019: F, t163: F) -> (F, F, F, F, F, F, F, F) {
    let t12169 = t786 * t10463;
    let t12179 = t10879 * t2015;
    let t12180 = t2013 * t12179;
    let t12198 = t786 * t10487;
    let t12234 = t2020 * t1849;
    let t12246 = t397 * t10791 * t786;
    let t12248 = 0.9994882620098509563e-2 * t782 * t12246;
    let t12251 = t2005 * t5477;
    let t12253 = t2019 * t2019;
    let t12254 = 1.0 / t12253;
    let t12261 = t397 * t163;
    (t12169, t12180, t12198, t12234, t12248, t12251, t12254, t12261)
}

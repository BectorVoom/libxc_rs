//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 594/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk594<F: Float>(t79: F, t8306: F, t534: F, t4391: F, t7706: F, t3952: F, t2059: F, t2326: F, t4400: F, t1312: F, t4406: F, t1581: F, t7710: F) -> (F, F, F, F, F, F, F, F, F) {
    let t8307 = t79 * t8306;
    let t8308 = t8307 * t534;
    let t8318 = t4391 * t7706;
    let t8319 = t3952 * t8318;
    let t8322 = t2059 * t2326;
    let t8323 = t4400 * t8322;
    let t8324 = t1312 * t8323;
    let t8327 = t4406 * t7706;
    let t8328 = t1312 * t8327;
    let t8331 = t1581 * t7710;
    (t8307, t8308, t8318, t8319, t8323, t8324, t8327, t8328, t8331)
}

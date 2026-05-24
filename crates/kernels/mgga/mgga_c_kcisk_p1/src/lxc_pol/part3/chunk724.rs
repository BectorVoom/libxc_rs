//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 724/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk724<F: Float>(t1862: F, t5060: F, t5064: F, t1869: F, t4811: F, t5065: F, t1689: F, t4822: F, t139: F, t5911: F, t710: F, t3521: F, t4606: F, sigma2: F) -> (F, F, F, F, F, F, F) {
    let t11236 = t1862 * t5060;
    let t11237 = t11236 * sigma2;
    let t11238 = t11237 * t5064;
    let t11239 = t1869 * t11238;
    let t11241 = t4811 * t5065;
    let t11245 = t1689 * t4822;
    let t11250 = t139 * t5911;
    let t11252 = F::cast_from(0.29201909629629629629e-3_f64) * t11250 * t710;
    let t11255 = t3521 * t4606;
    (t11236, t11239, t11241, t11245, t11250, t11252, t11255)
}

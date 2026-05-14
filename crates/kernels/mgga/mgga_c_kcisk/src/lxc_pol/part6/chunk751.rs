//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 751/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk751<F: Float>(t1610: F, t8432: F, t1149: F, t7724: F, t2527: F, t7715: F, t6666: F, t5184: F, t5182: F, t2364: F, t9019: F, t5192: F, t15862: F, t8485: F, t9035: F, t2441: F) -> (F, F, F, F, F, F, F, F) {
    let t28036 = t8432 * t1610;
    let t28152 = t7724 * t1149;
    let t28208 = t7715 * t2527;
    let t28209 = t6666 * t28208;
    let t28210 = t5184 * t28209;
    let t28211 = t5182 * t28210;
    let t28217 = t9019 * t2364;
    let t28218 = t5192 * t28217;
    let t28219 = t5182 * t28218;
    let t28221 = t15862 * t8485;
    let t28222 = t5182 * t28221;
    let t28224 = t9035 * t2364;
    let t28225 = t5192 * t28224;
    let t28226 = t5182 * t28225;
    let t28228 = t7715 * t2441;
    (t28036, t28152, t28208, t28211, t28219, t28222, t28226, t28228)
}

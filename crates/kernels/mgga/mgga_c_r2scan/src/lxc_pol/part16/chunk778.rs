//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 778/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk778<F: Float>(t1610: F, t2207: F, t2691: F, t2530: F, t537: F, t6217: F, t7460: F, t1632: F, t2634: F, t551: F, t2184: F, t2612: F) -> (F, F, F, F, F) {
    let t7500 = F::new(0.34930954652346593434e-1) * t2207 * t1610 * t2691;
    let t7503 = t537 * t2530;
    let t7512 = t6217 * t7460;
    let t7551 = t551 * t1632 * t2634;
    let t7553 = F::new(0.46230515946956099004e0) * t2184 * t7551;
    let t7555 = t551 * t1632 * t2612;
    (t7500, t7503, t7512, t7553, t7555)
}

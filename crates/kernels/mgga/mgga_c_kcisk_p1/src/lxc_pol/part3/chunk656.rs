//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 656/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk656<F: Float>(t10450: F, t1801: F, t1800: F, t1799: F, t4581: F, t5055: F, t5054: F, t1849: F, t579: F, t1336: F, t140: F, t4596: F, t694: F) -> (F, F, F, F, F) {
    let t10451 = t1801 * t10450;
    let t10452 = t1800 * t10451;
    let t10453 = t1799 * t10452;
    let t10455 = t4581 * t5055;
    let t10456 = t5054 * t10455;
    let t10459 = F::cast_from(1.0_f64) / t579 / t1849;
    let t10461 = t140 * t1336 * t10459;
    let t10463 = F::cast_from(1.0_f64) / t4596 / t694;
    (t10453, t10456, t10459, t10461, t10463)
}

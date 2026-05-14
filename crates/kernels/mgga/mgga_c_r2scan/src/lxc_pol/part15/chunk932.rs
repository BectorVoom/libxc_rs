//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 932/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk932<F: Float>(t19790: F, t481: F, t1541: F, t57: F, t2141: F, t3433: F, t2146: F, t2182: F, t146: F, t6091: F, t774: F, t537: F, t252: F, t545: F, t6394: F, t1415: F) -> (F, F, F, F, F, F, F, F) {
    let t19791 = t19790 * t481;
    let t19839 = t57 * t1541;
    let t19853 = t3433 * t2141;
    let t19865 = t2182 * t2146;
    let t19872 = t146 * t6091 * t774;
    let t19875 = t1541 * t537;
    let t19877 = t146 * t19875 * t252;
    let t19883 = t545 * t6394;
    let t20094 = t1415 * t57;
    (t19791, t19839, t19853, t19865, t19872, t19877, t19883, t20094)
}

//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 999/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk999<F: Float>(t11036: F, t2381: F, t2391: F, t3358: F, t1070: F, t8355: F, t3363: F, t8358: F, t2378: F, t3366: F, t3629: F, t6654: F) -> (F, F, F, F, F, F) {
    let t11868 = t11036 * t2381;
    let t11870 = t3358 * t2391;
    let t11872 = t8355 * t1070;
    let t11874 = t8358 * t3363;
    let t11876 = t2378 * t3366;
    let t11878 = t6654 * t3629;
    (t11868, t11870, t11872, t11874, t11876, t11878)
}

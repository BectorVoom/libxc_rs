//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1148/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1148<F: Float>(t2196: F, t25177: F, t3308: F, t3588: F, t37932: F, t10894: F, t8243: F, t10810: F, t2184: F, t7629: F, t7625: F, t26314: F, t37755: F, t39841: F) -> (F, F, F, F, F, F) {
    let t39975 = t2196 * t3308 * t25177;
    let t39977 = t37932 * t3588;
    let t39979 = t10894 * t8243;
    let t39982 = t2184 * t10810 * t7629;
    let t39984 = t10894 * t7625;
    let t39987 = t37755 * t39841 * t26314;
    (t39975, t39977, t39979, t39982, t39984, t39987)
}

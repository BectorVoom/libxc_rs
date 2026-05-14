//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1164/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1164<F: Float>(t34177: F, t415: F, t1790: F, t2464: F, t32935: F, t7261: F, t32948: F, t34073: F, t34148: F, t34154: F, t34162: F, t34165: F, t34168: F, t34171: F, t34175: F, t9652: F, t9664: F, t9922: F) -> (F, F, F, F) {
    let t34178 = t415 * t34177;
    let t34180 = t2464 * t1790;
    let t34181 = t32935 * t34180;
    let t34182 = t7261 * t34181;
    let t34185 = -0.10416666666666666667e-1 * t9664 * t34148 + 0.10416666666666666667e-1 * t34073 * t9652 + 0.40208333333333333335e-2 * t34154 * t9652 + 0.40208333333333333335e-2 * t32948 * t9922 + 0.49745833333333333332e-2 * t34162 + 0.66327777777777777776e-2 * t34165 - 0.44218518518518518517e-2 * t34168 + 0.16581944444444444444e-2 * t34171 - 0.24872916666666666666e-2 * t34175 + 0.16581944444444444444e-2 * t34178 - 0.10416666666666666667e-1 * t9664 * t34182;
    (t34178, t34181, t34182, t34185)
}

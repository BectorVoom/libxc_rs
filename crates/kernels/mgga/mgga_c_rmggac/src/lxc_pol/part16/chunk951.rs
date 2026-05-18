//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 951/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk951<F: Float>(t36457: F, t9835: F, t1469: F, t3369: F, t39851: F, t559: F, t2412: F, t8582: F, t2191: F, t9790: F, t9938: F, t10040: F) -> (F, F, F, F, F, F) {
    let t45832 = t36457 * t9835;
    let t45836 = t39851 * t3369 * t559 * t1469;
    let t45844 = t2412 * t8582;
    let t45846 = t2191 * t9790;
    let t45864 = t2191 * t9938;
    let t45866 = t2191 * t10040;
    (t45832, t45836, t45844, t45846, t45864, t45866)
}

//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1393/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1393<F: Float>(t34680: F, t109148: F, t109149: F, t111577: F, t111582: F, t111583: F, t111584: F, t111585: F, t116034: F, t116061: F, t116062: F, t116063: F, t116064: F, t118576: F, t118587: F, t118600: F, t118617: F, t32693: F, t32881: F, t32884: F, t35049: F, t8: F) -> (F,) {
    let t118621 = t34680 / 8.0;
    let t118622 = t109148 + t116061 - t32881 - t32884 - t109149 - t116062 - t116063 + t111577 + t116034 - t116064 + t35049 + t8 * (t118576 + t118587 + t118600 + t118617) + t32693 + t111582 - t111583 - t118621 - t111584 - t111585;
    (t118622,)
}

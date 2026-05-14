//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1405/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1405<F: Float>(t111223: F, t111224: F, t111472: F, t111507: F, t111509: F, t111512: F, t111515: F, t111518: F, t1152: F, t116014: F, t35046: F, t35050: F, t35053: F, t33986: F, t109149: F, t111577: F, t111582: F, t111583: F, t111584: F, t111585: F, t116032: F, t116033: F, t116036: F, t116037: F, t116038: F, t116039: F, t118633: F, t120903: F, t120918: F, t32686: F, t35052: F, t35058: F, t35065: F, t8: F) -> (F,) {
    let t120925 = -t111223 + t1152 * t35053 / 8.0 - t111224 + t111472 + t116014 + t1152 * t35050 / 16.0 - t111507 + t111509 + t111512 - t111515 + t111518 + t1152 * t35046 / 16.0;
    let t120929 = 4.0 * t33986;
    let t120930 = -t35052 - t109149 - t116032 - t116033 + t111577 + t32686 + t8 * (t118633 + t120903 + t120918 + t120925) + t116036 - t35058 + t120929 - t116037 + t111582 - t116038 - t111583 - t35065 - t111584 - t111585 - t116039;
    (t120930,)
}

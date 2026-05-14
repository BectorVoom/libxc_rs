//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1446/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1446<F: Float>(t33339: F, t33343: F, t33982: F, t32685: F, t33984: F, t33988: F, t33991: F, t33994: F, t111582: F, t111583: F, t111584: F, t111585: F, t32692: F, t32693: F, t35049: F, t109148: F, t109149: F, t111577: F, t116030: F, t31892: F, t32542: F, t32689: F) -> (F,) {
    let t116031 = t33339 / 8.0;
    let t116032 = t33343 / 8.0;
    let t116033 = t33982 / 8.0;
    let t116034 = 4.0 * t32685;
    let t116036 = t33984 / 8.0;
    let t116037 = t33988 / 8.0;
    let t116038 = t33991 / 8.0;
    let t116039 = t33994 / 8.0;
    let t116040 = t116036 + t35049 - t116037 - t32692 + t32693 + t111582 - t116038 - t111583 - t111584 - t111585 - t116039;
    let tv4rho3sigma3 = t116030 - t116031 - t31892 + t109148 - t109149 - t32542 - t116032 - t116033 + t111577 + t116034 - t32689 + t116040;
    (tv4rho3sigma3,)
}

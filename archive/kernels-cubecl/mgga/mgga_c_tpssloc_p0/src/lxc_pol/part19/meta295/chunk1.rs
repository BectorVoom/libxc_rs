//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1077/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1077<F: Float>(t13004: F, t205: F, t4126: F, t782: F, t68: F, t822: F, t2644: F, t820: F, t2617: F, t4177: F, t2628: F, t836: F) -> (F, F, F, F, F, F) {
    let t13005 = t205 * t13004;
    let t13012 = t782 * t4126;
    let t13151 = t822 * t68;
    let t13222 = t2644 * t820;
    let t13254 = t2617 * t4177;
    let t13257 = t2628 * t836;
    (t13005, t13012, t13151, t13222, t13254, t13257)
}

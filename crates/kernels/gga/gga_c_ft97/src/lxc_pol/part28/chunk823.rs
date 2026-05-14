//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 823/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk823<F: Float>(t12374: F, t5820: F, t6608: F, t94765: F, t23809: F, t3347: F, t12411: F, t23724: F, t6604: F, t1391: F, t2101: F, t582: F, t5935: F, t6685: F, t157: F, t40266: F) -> (F, F, F, F, F, F, F, F, F) {
    let t104860 = t12374 * t5820;
    let t105080 = t94765 * t6608;
    let t105135 = t3347 * t23809;
    let t105260 = t12411 * t23809;
    let t105279 = t23724 * t6604;
    let t106296 = t2101 * t1391;
    let t106300 = t582 * t5935;
    let t106551 = t582 * t6685;
    let t106555 = t40266 * t157;
    (t104860, t105080, t105135, t105260, t105279, t106296, t106300, t106551, t106555)
}

//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 645/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk645<F: Float>(t10798: F, t5016: F, t5013: F, t1782: F, t7233: F, t1785: F, t4640: F, t4998: F, t5021: F, t1773: F, t10487: F, t662: F, t10441: F, t5006: F, t1772: F, t4983: F) -> (F, F, F, F, F) {
    let t10799 = t10798 * t5016;
    let t10800 = t5013 * t10799;
    let t10802 = t7233 * t1782;
    let t10803 = t4640 * t1785;
    let t10804 = t10802 * t10803;
    let t10809 = t4998 * t5021;
    let t10810 = t1773 * t10809;
    let t10812 = t662 * t10487;
    let t10813 = t10812 * t10441;
    let t10814 = t5006 * t10813;
    let t10817 = t4983 * t1772;
    (t10800, t10804, t10810, t10814, t10817)
}

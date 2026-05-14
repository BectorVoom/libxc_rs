//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 687/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk687<F: Float>(t1147: F, t2156: F, t1306: F, t135: F, t273: F, t2745: F, t2748: F, t2750: F, t2753: F, t2785: F, t2789: F, t2857: F, t2859: F, t2862: F, t2864: F, t2868: F, t2872: F, t2877: F, t2993: F, t803: F, t805: F) -> (F, F) {
    let t2997 = t1147 * t2156;
    let t3000 = t135 * t273 * t2993 * t805 - t1306 * t2997 * t803 - t2745 + t2748 + t2750 - t2753 + t2785 + t2789 + t2857 + t2859 - t2862 - t2864 + t2868 - t2872 - t2877;
    (t2997, t3000)
}

//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 751/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk751<F: Float>(t1519: F, t798: F, t1496: F, t2563: F, t1495: F, t210: F, t776: F, t119: F, t4119: F, t225: F, t4142: F) -> (F, F, F, F, F) {
    let t4149 = t798 * t1519;
    let t4152 = t2563 * t1496;
    let t4155 = t210 * t1495 * t776;
    let t4158 = t119 * t4119;
    let t4159 = t210 * t4158;
    let t4162 = t4142 * t225;
    (t4149, t4152, t4155, t4159, t4162)
}

//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 545/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk545<F: Float>(t2697: F, t849: F, t1891: F, t241: F, t67: F, t225: F, t853: F, t257: F, t856: F, t68: F, t252: F, t2627: F) -> (F, F, F, F, F) {
    let t2698 = t2697 * t849;
    let t2700 = t241 * t1891;
    let t2701 = t2700 * t67;
    let t2713 = t853 * t225;
    let t2717 = F::cast_from(1.0_f64) / t856 / t257;
    let t2718 = t68 * t2717;
    let t2728 = t2627 * t252;
    (t2698, t2701, t2713, t2718, t2728)
}

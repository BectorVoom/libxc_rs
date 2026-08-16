//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2080/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2080<F: Float>(t23637: F, t82822: F, t1920: F, t23620: F, t968: F, t23617: F, t6680: F, t10454: F, t6765: F, t10889: F, t3033: F, t6753: F) -> (F, F, F, F, F) {
    let t82823 = t82822 * t23637;
    let t82828 = t1920 * t968 * t23620;
    let t82830 = t6680 * t23617;
    let t82843 = t6765 * t10454;
    let t82848 = t3033 * t6753 * t10889;
    (t82823, t82828, t82830, t82843, t82848)
}

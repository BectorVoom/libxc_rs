//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1326/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1326<F: Float>(t816: F, t9612: F, t2632: F, t776: F, t2678: F, t815: F, t836: F, t812: F, t2649: F, t2617: F, t2642: F, t1891: F, t67: F) -> (F, F, F, F, F, F, F) {
    let t9613 = t9612 * t816;
    let t9627 = t2632 * t776;
    let t9632 = t2632 * t2678;
    let t9637 = t815 * t836;
    let t9638 = t812 * t9637;
    let t9639 = t9638 * t2649;
    let t9642 = t2617 * t2642;
    let t9645 = t1891 * t67;
    (t9613, t9627, t9632, t9638, t9639, t9642, t9645)
}

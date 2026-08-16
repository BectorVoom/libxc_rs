//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1575/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1575<F: Float>(t23122: F, t23124: F, t2617: F, t6620: F, t6619: F, t835: F, t812: F) -> (F, F, F, F) {
    let t23125 = t23122 * t23124;
    let t23127 = t2617 * t6620;
    let t23132 = t6619 * t835;
    let t23133 = t812 * t23132;
    (t23125, t23127, t23132, t23133)
}

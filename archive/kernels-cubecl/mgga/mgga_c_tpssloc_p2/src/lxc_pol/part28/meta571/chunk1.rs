//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1852/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1852<F: Float>(t25132: F, t81876: F, t13336: F, t1898: F, t249: F, t23047: F, t4166: F, t2635: F, t1516: F, t81766: F, t23127: F, t4261: F) -> (F, F, F, F, F) {
    let t87213 = t81876 * t25132;
    let t87216 = t13336 * t1898 * t249;
    let t87218 = t4166 * t23047;
    let t87219 = t87218 * t2635;
    let t87222 = t81766 * t1516;
    let t87224 = t23127 * t4261;
    (t87213, t87216, t87219, t87222, t87224)
}

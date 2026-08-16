//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 956/1049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk956<F: Float>(t5612: F, t6605: F, t6612: F, t23046: F, t5585: F, t1894: F, t23078: F, t5527: F, t59: F, t5624: F, t8343: F, t23097: F, t28395: F) -> (F, F, F, F, F) {
    let t126325 = t6605 * t6612 * t5612;
    let t126328 = t6605 * t23046 * t5585;
    let t126332 = t23078 * t1894 * t59 * t5527;
    let t126334 = t8343 * t5624;
    let t126337 = t23097 * t6612 * t28395;
    (t126325, t126328, t126332, t126334, t126337)
}

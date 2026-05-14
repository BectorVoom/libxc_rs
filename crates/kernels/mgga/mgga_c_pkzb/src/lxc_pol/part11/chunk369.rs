//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 369/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk369<F: Float>(t1480: F, t1483: F, t1486: F, t1490: F, t1492: F, t1495: F) -> (F,) {
    let t1527 = -0.42198333333333333333e0 * t1480 + 0.84396666666666666666e0 * t1483 + 0.39862222222222222223e0 * t1486 + 0.68258333333333333333e-1 * t1490 + 0.13651666666666666667e0 * t1492 + 0.13692777777777777778e0 * t1495;
    (t1527,)
}

//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 522/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk522<F: Float>(t4623: F, t4624: F, t706: F, t574: F, t673: F, t1648: F, t682: F, t1824: F, t298: F, t446: F, t569: F) -> (F, F, F, F, F, F, F) {
    let t4625 = t4623 * t4624;
    let t4626 = t706 * t4625;
    let t4629 = t673 * t574;
    let t4630 = t682 * t1648;
    let t4631 = t4630 * t1824;
    let t4632 = t4629 * t4631;
    let t4636 = t298 * t446 * t569;
    (t4625, t4626, t4629, t4630, t4631, t4632, t4636)
}

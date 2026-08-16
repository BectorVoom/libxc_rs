//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1406/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1406<F: Float>(t10235: F, t10237: F, t2987: F, t3008: F, t2990: F, t2250: F, t2989: F) -> (F, F, F, F) {
    let t10238 = t10235 * t10237;
    let t10241 = t2987 * t3008;
    let t10242 = t10241 * t2990;
    let t10245 = t2989 * t2250;
    (t10238, t10241, t10242, t10245)
}

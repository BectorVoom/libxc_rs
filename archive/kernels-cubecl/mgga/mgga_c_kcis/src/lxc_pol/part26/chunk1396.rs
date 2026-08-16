//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1396/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1396<F: Float>(t101774: F, t103957: F, t29664: F, t29667: F, t29670: F, t29672: F, t29674: F, t91785: F, t91786: F, t97626: F, t99790: F, t99791: F) -> F {
    let tv4rho3sigma8 = t101774 - t91785 - t97626 + t91786 - t29664 - t29667 - t29670 + t29672 - t99790 + t29674 - t99791 + t103957;
    tv4rho3sigma8
}

//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1769/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1769<F: Float>(t24826: F, t7378: F, t7319: F, t7327: F, t1170: F, t7381: F, t2121: F, t210: F, t7371: F) -> (F, F, F, F, F) {
    let t24827 = t24826 * t7378;
    let t24833 = t7319 * t7327;
    let t24844 = t1170 * t7381;
    let t24845 = t2121 * t24844;
    let t24847 = t7371 * t210;
    (t24827, t24833, t24844, t24845, t24847)
}

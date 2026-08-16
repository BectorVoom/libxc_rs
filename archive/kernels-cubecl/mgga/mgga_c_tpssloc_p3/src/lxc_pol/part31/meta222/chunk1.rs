//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 957/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk957<F: Float>(t349: F, t5914: F, t1634: F, t3174: F, t381: F, t5872: F) -> (F, F, F, F) {
    let t5915 = t349 * t5914;
    let t5919 = t1634 * t1634;
    let t5920 = t3174 * t5919;
    let t5928 = t381 * t5872;
    (t5915, t5919, t5920, t5928)
}

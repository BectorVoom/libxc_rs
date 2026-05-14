//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 677/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk677<F: Float>(t10660: F, t10664: F, t11352: F, t11355: F, t11358: F, t11361: F, t11382: F, t1648: F, t1815: F, t4624: F, t4652: F, t4664: F, t4667: F, t574: F, t1887: F, t706: F) -> (F, F, F) {
    let t11385 = 3.0 / 16.0 * t11352 * t10664 - 3.0 / 8.0 * t11355 * t4624 - 3.0 / 8.0 * t4664 * t11358 + 3.0 / 4.0 * t11361 * t1648 + 3.0 / 4.0 * t4667 * t4652 + t1815 * t10660 / 4.0 + t574 * t11382 / 2.0;
    let t11386 = t1887 * t11385;
    let t11387 = t706 * t11386;
    (t11385, t11386, t11387)
}

//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 768/1013 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk768<F: Float>(t107: F, t39048: F, t787: F, t12251: F, t1980: F, t12161: F, t296: F, t1: F, t2021: F, t2610: F, t38912: F, t1381: F, t3699: F) -> (F, F, F, F, F, F) {
    let t39050 = t787 * t39048 * t107;
    let t39118 = t1980 * t12251;
    let t39121 = t296 * t12161;
    let t39123 = t787 * t39121 * t1;
    let t39145 = t39048 * t1;
    let t39146 = t2021 * t39145;
    let t39149 = t2610 * t38912;
    let t39337 = t3699 * t1381;
    (t39050, t39118, t39123, t39146, t39149, t39337)
}

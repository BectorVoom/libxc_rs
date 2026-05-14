//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1114/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1114<F: Float>(t23132: F, t92: F, t1286: F, t22499: F, t376: F, t1349: F, t24060: F, t1984: F, t23884: F, t1637: F, t5848: F, t24130: F, t5844: F, t24116: F, t5766: F, t5780: F) -> (F, F, F, F, F, F, F, F, F) {
    let t94089 = t23132 * t92;
    let t94104 = t1286 * t376 * t22499;
    let t94148 = t1349 * t376 * t24060;
    let t94155 = t1984 * t23884;
    let t94175 = t1349 * t1637 * t5848;
    let t94184 = t1349 * t376 * t24130;
    let t94191 = t1349 * t1637 * t5844;
    let t94198 = t5766 * t24116;
    let t94201 = t1349 * t1637 * t5780;
    (t94089, t94104, t94148, t94155, t94175, t94184, t94191, t94198, t94201)
}

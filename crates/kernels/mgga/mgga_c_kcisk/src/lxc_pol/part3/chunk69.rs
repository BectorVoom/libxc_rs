//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 69/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk69<F: Float>(t139: F, t175: F, t197: F, t198: F, t201: F, t190: F, t116: F, t167: F) -> (F, F, F) {
    let t205 = F::new(0.619125e-2) * t197 * t198 - F::new(0.79593333333333333331e-1) * t139 * t201 * t175;
    let t206 = t205 * t190;
    let t207 = t116 * t167;
    (t205, t206, t207)
}

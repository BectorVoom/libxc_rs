//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 578/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk578<F: Float>(t262: F, t7590: F, t2118: F, t655: F, t7581: F, t265: F, t321: F, t793: F, t27: F, t3814: F) -> (F, F, F, F, F, F) {
    let t7591 = t262 * t7590;
    let t7592 = t2118 * t7591;
    let t7594 = t655 * t7581;
    let t7595 = F::new(0.11111899192470324408e-1) * t7594;
    let t7596 = t265 * t321;
    let t7597 = t793 * t7596;
    let t7599 = t3814 * t27;
    (t7591, t7592, t7595, t7596, t7597, t7599)
}

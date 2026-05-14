//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 244/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk244<F: Float>(t227: F, t1060: F, t565: F, t298: F, t430: F, t569: F, zeta_threshold: F) -> (F, F, F) {
    let t228 = t227 <= zeta_threshold;
    let t1628 = piecewise3(t228, 0.0, t1060);
    let t1629 = t565 * t1628;
    let t1634 = t298 * t430 * t569;
    (t1628, t1629, t1634)
}

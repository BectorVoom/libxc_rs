//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 110/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk110<F: Float>(t311: F, t312: F, t579: F, t571: F, t574: F, t577: F) -> (F, F, F, F) {
    let t581 = t311 * t312 * t579;
    let t583 = 0.379785e1 * t574 + 0.8969e0 * t571 + 0.204775e0 * t577 + 0.123235e0 * t581;
    let t586 = 1.0 + 0.16081824322151104822e2 / t583;
    let t587 = f64::ln(t586);
    (t581, t583, t586, t587)
}

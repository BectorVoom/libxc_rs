//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1101/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1101<F: Float>(t32401: F, t9516: F, t9515: F, t9523: F, t4419: F, t9543: F, t2737: F, t9528: F, t9511: F, t9535: F) -> (F, F, F, F, F, F, F) {
    let t32402 = t9516 * t32401;
    let t32417 = t9515 * t9523;
    let t32422 = t4419 * t9543;
    let t32423 = t2737 * t32422;
    let t32425 = t2737 * t32401;
    let t32433 = t9515 * t9528;
    let t32436 = t9511 * t9535;
    (t32402, t32417, t32422, t32423, t32425, t32433, t32436)
}

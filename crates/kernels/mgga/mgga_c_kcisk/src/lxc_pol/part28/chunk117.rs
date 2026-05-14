//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 117/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk117<F: Float>(t606: F, t25: F, t353: F, t579: F, t609: F) -> (F, F, F, F, F) {
    let t612 = pow_3_2(t606);
    let t615 = t353 * t25 * t579;
    let t617 = 0.379785e1 * t609 + 0.8969e0 * t606 + 0.204775e0 * t612 + 0.24647e0 * t615;
    let t620 = 1.0 + 0.16081824322151104822e2 / t617;
    let t621 = f64::ln(t620);
    (t612, t615, t617, t620, t621)
}

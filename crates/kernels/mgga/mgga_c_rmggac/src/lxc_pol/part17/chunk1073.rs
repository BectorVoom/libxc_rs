//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 1073/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk1073<F: Float>(t1704: F, t503: F, t681: F, t1971: F, t495: F, t511: F, t8517: F, t9969: F, t41914: F, t8571: F, t40031: F, t40092: F) -> (F, F, F, F, F) {
    let t47587 = t503 * t1704;
    let t47588 = t47587 * t681;
    let t47594 = t8517 * t1971 * t511 * t9969 * t495;
    let t47596 = t8571 * t41914;
    let t47598 = t8571 * t40031;
    let t47600 = t8571 * t40092;
    (t47588, t47594, t47596, t47598, t47600)
}

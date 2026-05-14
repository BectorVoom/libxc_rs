//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 934/963 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk934<F: Float>(t47587: F, t681: F, t1971: F, t495: F, t511: F, t8517: F, t9969: F, t41914: F, t8571: F, t40031: F, t40092: F, t41576: F, t236: F, t618: F, t1981: F, t3134: F, t8512: F) -> (F, F, F, F, F, F, F) {
    let t47588 = t47587 * t681;
    let t47594 = t8517 * t1971 * t511 * t9969 * t495;
    let t47596 = t8571 * t41914;
    let t47598 = t8571 * t40031;
    let t47600 = t8571 * t40092;
    let t47602 = t8571 * t41576;
    let t47604 = t236 * t618;
    let t47607 = t8512 * t1981 * t3134 * t47604;
    (t47588, t47594, t47596, t47598, t47600, t47602, t47607)
}

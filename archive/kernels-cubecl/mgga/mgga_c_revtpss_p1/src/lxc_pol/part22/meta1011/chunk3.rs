//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3472/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3472<F: Float>(t63894: F, t63898: F, t63906: F, t63916: F, t63918: F, t63920: F, t63923: F, t63925: F, t63927: F, t63929: F, t63934: F, t63937: F) -> F {
    let t65395 = t63894 - t63898 - t63906 - t63916 - t63918 - t63920 + t63923 - t63925 - t63927 + t63929 - t63934 + t63937;
    t65395
}

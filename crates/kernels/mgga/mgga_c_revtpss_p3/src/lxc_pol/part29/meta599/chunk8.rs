//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 2047/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2047<F: Float>(t28944: F, t575: F, t104071: F, t104073: F, t104077: F, t104079: F, t104081: F, t104083: F, t1456: F, t1914: F, t26743: F, t28993: F, t5790: F, t5808: F, t7542: F, t7560: F, t95196: F, t96633: F) -> F {
    let t104085 = F::new(2.0) * t28944 * t575;
    let t104087 = F::new(2.0) * t1456 * t28993 + t1914 * t26743 + F::new(2.0) * t5790 * t7560 + F::new(2.0) * t5808 * t7542 + t104071 + t104073 + t104077 + t104079 + t104081 + t104083 + t104085 + t95196 + t96633;
    t104087
}

//! MGGA_C_REVTPSS lxc pol kernel — _part41_v4rho3tau_4 meta423 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1481;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_meta423<F: Float>(t21881: F, t93: F, t10208: F, t625: F, t46157: F, t69: F, t2289: F, t2339: F, t8260: F, t655: F, t8269: F, t2349: F, t43: F) -> (F, F, F, F, F, F, F, F) {
        let (t109242, t116912, t116919, t116926, t116927, t116929, t116930, t116942) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1481::<F>(t21881, t93, t10208, t625, t46157, t69, t2289, t2339, t8260, t655, t8269, t2349, t43);
    (t109242, t116912, t116919, t116926, t116927, t116929, t116930, t116942)
}

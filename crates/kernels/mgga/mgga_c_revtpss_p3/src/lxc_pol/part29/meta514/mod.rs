//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta514 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1836;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta514<F: Float>(t10709: F, t25227: F, t2661: F, t240: F, t25260: F, t10728: F, t2479: F, t25222: F, t25228: F, t9775: F, t10732: F, t10705: F, t25234: F) -> (F, F, F, F, F, F, F) {
        let (t93080, t93082, t93084, t93086, t93088, t93091, t93095) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1836::<F>(t10709, t25227, t2661, t240, t25260, t10728, t2479, t25222, t25228, t9775, t10732, t10705, t25234);
    (t93080, t93082, t93084, t93086, t93088, t93091, t93095)
}

//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta370 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1914;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1915;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta370<F: Float>(t12966: F, t480: F, t12657: F, t225: F, t3667: F, t3678: F, t1236: F, t371: F, t676: F, t1235: F, t12627: F) -> (F, F, F, F, F, F, F) {
        let (t12967, t12975) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1914::<F>(t12966, t480, t12657, t225);
        let (t12976, t12979, t12984, t12985, t12987) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1915::<F>(t12975, t480, t3667, t3678, t1236, t371, t676, t1235, t12627, t225);
    (t12967, t12975, t12976, t12979, t12984, t12985, t12987)
}

//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta496 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2227;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta496<F: Float>(t16313: F, t3066: F, t1695: F, t3325: F, t3269: F, t3270: F, t11121: F, t5015: F, t999: F, t1079: F, t342: F, t4930: F) -> (F, F, F, F, F, F, F, F) {
        let (t16314, t16317, t16318, t16321, t16322, t16327, t16328, t16333) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2227::<F>(t16313, t3066, t1695, t3325, t3269, t3270, t11121, t5015, t999, t1079, t342, t4930);
    (t16314, t16317, t16318, t16321, t16322, t16327, t16328, t16333)
}

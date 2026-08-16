//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta618 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2525;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2526;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta618<F: Float>(t15957: F, t6266: F, t3092: F, t16509: F, t4891: F, t16584: F) -> (F, F, F, F) {
        let (t19730, t19731, t19738) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2525::<F>(t15957, t6266, t3092, t16509, t4891);
        let t19741 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2526::<F>(t16584, t4891);
    (t19730, t19731, t19738, t19741)
}

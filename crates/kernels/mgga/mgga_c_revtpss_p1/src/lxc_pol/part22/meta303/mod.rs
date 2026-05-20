//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta303 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1738;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta303<F: Float>(t10073: F, t4089: F, t1398: F, t1419: F, t4086: F, t543: F, t2782: F, t4056: F, t555: F, t9990: F, t1432: F, t2470: F, t4107: F) -> (F, F, F, F, F, F, F) {
        let (t10074, t10079, t10080, t10084, t10085, t10090, t10098) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1738::<F>(t10073, t4089, t1398, t1419, t4086, t543, t2782, t4056, t555, t9990, t1432, t2470, t4107);
    (t10074, t10079, t10080, t10084, t10085, t10090, t10098)
}

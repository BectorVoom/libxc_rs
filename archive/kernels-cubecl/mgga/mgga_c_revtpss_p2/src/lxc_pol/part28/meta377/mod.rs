//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta377 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1428;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta377<F: Float>(t9404: F, t2626: F, t5571: F, t1856: F, t2608: F, t512: F, t9408: F, t9411: F, t9422: F, t9429: F, t13612: F, t13615: F, t13620: F, t13622: F, t13623: F, t13624: F, t13625: F, t4139: F, t4140: F, t5536: F, t5542: F, t5627: F, t9394: F, t9415: F, t9421: F, t9427: F, t9546: F) -> (F, F, F, F, F, F, F, F) {
        let (t13629, t13631, t13633, t13634, t13635, t13636, t13637, t13638) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1428::<F>(t9404, t2626, t5571, t1856, t2608, t512, t9408, t9411, t9422, t9429, t13612, t13615, t13620, t13622, t13623, t13624, t13625, t4139, t4140, t5536, t5542, t5627, t9394, t9415, t9421, t9427, t9546);
    (t13629, t13631, t13633, t13634, t13635, t13636, t13637, t13638)
}

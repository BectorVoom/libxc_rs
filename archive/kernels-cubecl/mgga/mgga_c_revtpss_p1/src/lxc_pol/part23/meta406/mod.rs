//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta406 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1779;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1780;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta406<F: Float>(t670: F, t6765: F, t1843: F, t4292: F, t1310: F, t5920: F, t116: F, t5876: F, t4343: F, t4542: F, t2404: F, t5966: F, t14613: F, t162: F, t4403: F, t14312: F, t5940: F, t705: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t18232, t18235, t18242, t18245) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1779::<F>(t670, t6765, t1843, t4292, t1310, t5920, t116, t5876);
        let (t18253, t18256, t18259, t18261, t18262, t18263) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1780::<F>(t4343, t4542, t2404, t5966, t14613, t162, t4403, t14312, t5940, t705);
    (t18232, t18235, t18242, t18245, t18253, t18256, t18259, t18261, t18262, t18263)
}

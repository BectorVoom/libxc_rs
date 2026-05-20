//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta926 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3149;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3150;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta926<F: Float>(t17351: F, t17354: F, t56756: F, t3588: F, t3611: F, t12904: F, t5293: F, t12959: F, t17569: F, t11262: F, t1261: F, t5269: F, t17236: F, t3172: F, t17540: F, t3711: F, t12956: F, t17209: F, t17198: F, t12773: F, t17605: F, t17557: F, t17535: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t56758, t56760, t56785, t56787, t56790) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3149::<F>(t17351, t17354, t56756, t3588, t3611, t12904, t5293, t12959, t17569, t11262, t1261, t5269);
        let (t56793, t56796, t56798, t56812, t56835, t56838, t56853) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3150::<F>(t1261, t17236, t3172, t17540, t3711, t12956, t17209, t17198, t12773, t17605, t17557, t17535);
    (t56758, t56760, t56785, t56787, t56790, t56793, t56796, t56798, t56812, t56835, t56838, t56853)
}

//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta300 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1183;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1184;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta300<F: Float>(t409: F, t416: F, t1134: F, t3391: F, t406: F, t12252: F, t12259: F, t12261: F, t12263: F, t12265: F, t12271: F, t12275: F, t12279: F, t12284: F, t12289: F, t12292: F, t12323: F, t3390: F, t3399: F, t3407: F, t12295: F, t11335: F, t281: F, t414: F, t1139: F, t12322: F, t12297: F, t12299: F, t12301: F, t12303: F, t12307: F, t12310: F, t12314: F, t12317: F, t12320: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t12327, t12329, t12331, t12332, t12334) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1183::<F>(t409, t416, t1134, t3391, t406, t12252, t12259, t12261, t12263, t12265, t12271, t12275, t12279, t12284, t12289, t12292, t12323);
        let (t12343, t12344, t12346, t12347, t12351, t12354, t12356) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1184::<F>(t1134, t3390, t3399, t3407, t12295, t11335, t281, t414, t1139, t12322, t12297, t12299, t12301, t12303, t12307, t12310, t12314, t12317, t12320);
    (t12327, t12329, t12331, t12332, t12334, t12343, t12344, t12346, t12347, t12351, t12354, t12356)
}

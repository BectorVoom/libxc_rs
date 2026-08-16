//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta654 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2105;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2106;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta654<F: Float>(t2143: F, t3566: F, t17306: F, t2142: F, t3556: F, t8945: F, t12640: F, t7635: F, t29313: F, t3801: F, t12587: F, t8220: F, t29468: F, t575: F, t1464: F, t8240: F, t1921: F, t7690: F, t2167: F, t5808: F, t2172: F, t5789: F, t1913: F, t7700: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t105576, t105579, t105598, t105644, t105665, t105669) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2105::<F>(t2143, t3566, t17306, t2142, t3556, t8945, t12640, t7635, t29313, t3801, t12587, t8220);
        let (t105792, t105794, t105796, t105798, t105800, t105802) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2106::<F>(t29468, t575, t1464, t8240, t1921, t7690, t2167, t5808, t2172, t5789, t1913, t7700);
    (t105576, t105579, t105598, t105644, t105665, t105669, t105792, t105794, t105796, t105798, t105800, t105802)
}

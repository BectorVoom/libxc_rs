//! MGGA_C_REVTPSS lxc pol kernel — _part39_v4rho3tau_2 meta415 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1504;
use chunk1::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1505;
use chunk2::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1506;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_meta415<F: Float>(t1913: F, t8302: F, t2192: F, t5789: F, t116890: F, t117095: F, t117369: F, t117374: F, t117720: F, t117765: F, t1458: F, t1464: F, t18178: F, t1921: F, t31088: F, t31329: F, t4154: F, t4168: F, t5790: F, t8373: F, t8389: F, t2184: F, t5808: F, t31328: F, t575: F, t8283: F, t1455: F, t116899: F, t117090: F, t117097: F, t117099: F, t117713: F, t1456: F, t18217: F, t1914: F, t2185: F, t3: F, t31127: F, t31377: F, t8284: F) -> F {
        let t117777 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1504::<F>(t1913, t8302, t2192, t5789, t116890, t117095, t117369, t117374, t117720, t117765, t1458, t1464, t18178, t1921, t31088, t31329, t4154, t4168, t5790, t8373, t8389);
        let t117796 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1505::<F>(t2184, t5808, t31328, t575, t1921, t8283, t1455, t8389, t116899, t117090, t117097, t117099, t117713, t1456, t18217, t1914, t2185, t3, t31127, t31377, t8284);
        let tv4rho3tau2 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1506::<F>(t117777, t117796);
    tv4rho3tau2
}

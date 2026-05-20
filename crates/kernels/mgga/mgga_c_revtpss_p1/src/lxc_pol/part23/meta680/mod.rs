//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta680 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2420;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2421;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta680<F: Float>(t1121: F, t13045: F, t606: F, t221: F, t461: F, t462: F, t624: F, t1250: F, t1235: F, t1236: F, t2434: F, t371: F, t12625: F, t458: F, t456: F, t225: F, t43813: F, t126: F, t13099: F, t1224: F, t12268: F, t1222: F, t1226: F, t2438: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t44738, t44797, t44799, t44829) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2420::<F>(t1121, t13045, t606, t221, t461, t462, t624, t1250, t1235, t1236, t2434, t371);
        let (t44842, t44843, t44865, t44895, t44919, t44931) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2421::<F>(t12625, t458, t456, t225, t43813, t126, t13099, t1224, t12268, t1222, t1226, t2438);
    (t44738, t44797, t44799, t44829, t44842, t44843, t44865, t44895, t44919, t44931)
}

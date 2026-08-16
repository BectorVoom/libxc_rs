//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta494 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1974;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta494<F: Float>(t1151: F, t20629: F, t16835: F, t1733: F, t5063: F, t5105: F, t12361: F, t6439: F, t3379: F, t6471: F, t12429: F, t12470: F, t17032: F, t20606: F, t20609: F, t20612: F, t20615: F, t20619: F, t20622: F, t20626: F, t3452: F, t3477: F, t5147: F) -> (F, F, F, F, F, F) {
        let (t20631, t20633, t20635, t20637, t20639, t20640) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1974::<F>(t1151, t20629, t16835, t1733, t5063, t5105, t12361, t6439, t3379, t6471, t12429, t12470, t17032, t20606, t20609, t20612, t20615, t20619, t20622, t20626, t3452, t3477, t5147);
    (t20631, t20633, t20635, t20637, t20639, t20640)
}

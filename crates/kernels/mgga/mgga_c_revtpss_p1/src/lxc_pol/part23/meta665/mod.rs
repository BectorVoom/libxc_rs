//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta665 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2396;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta665<F: Float>(t225: F, t42066: F, t41306: F, t367: F, t371: F, t373: F, t9291: F, t11773: F, t11865: F, t42051: F, t366: F, t1025: F, t1026: F, t2434: F) -> (F, F, F, F, F, F, F) {
        let (t42067, t42078, t42121, t42155, t42261, t42262, t42274) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2396::<F>(t225, t42066, t41306, t367, t371, t373, t9291, t11773, t11865, t42051, t366, t1025, t1026, t2434);
    (t42067, t42078, t42121, t42155, t42261, t42262, t42274)
}

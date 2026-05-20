//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta274 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1128;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1129;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta274<F: Float>(t3111: F, t3188: F, t3075: F, t999: F, t247: F, t3116: F, t11173: F, t373: F, t371: F, t372: F, t3211: F, t3215: F, t1026: F, t676: F, t1025: F) -> (F, F, F, F, F, F, F, F) {
        let (t11802, t11804) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1128::<F>(t3111, t3188, t3075, t999);
        let (t11806, t11809, t11811, t11814, t11817, t11818) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1129::<F>(t11804, t247, t3116, t11173, t373, t371, t372, t3211, t3215, t1026, t676, t1025);
    (t11802, t11804, t11806, t11809, t11811, t11814, t11817, t11818)
}

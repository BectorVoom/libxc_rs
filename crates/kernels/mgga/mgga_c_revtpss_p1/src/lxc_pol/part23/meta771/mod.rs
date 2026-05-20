//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta771 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2573;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2574;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta771<F: Float>(t57421: F, t1235: F, t371: F, t5318: F, t676: F, t225: F, t56331: F, t1789: F, t2434: F, t1012: F, t44958: F, t13026: F, t140: F, t1222: F, t1224: F, t5052: F, t697: F, t1260: F, t44843: F, t343: F, t56: F, t816: F, t65: F, t12256: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t57422, t57464, t57465, t57471, t57480, t57484) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2573::<F>(t57421, t1235, t371, t5318, t676, t225, t56331, t1789, t2434, t1012, t44958, t13026, t140);
        let (t57491, t57520, t57548, t57550) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2574::<F>(t1222, t1224, t5052, t697, t1260, t44843, t343, t56, t816, t13026, t65, t12256);
    (t57422, t57464, t57465, t57471, t57480, t57484, t57491, t57520, t57548, t57550)
}

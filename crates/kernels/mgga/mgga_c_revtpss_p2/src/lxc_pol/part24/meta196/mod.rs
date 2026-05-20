//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta196 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk927;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk928;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk929;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk930;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta196<F: Float>(t2482: F, t27: F, t4000: F, t555: F, t5744: F, t786: F, t4083: F, t9303: F, t123: F, t212: F, t2434: F, t138: F, t2438: F, t785: F, t9990: F, t1432: F, t1433: F, t9288: F, t225: F, t9646: F, t1428: F, t22: F, t2452: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t10001, t10022, t10023, t10035, t10069) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk927::<F>(t2482, t27, t4000, t555, t5744, t786, t4083, t9303, t123, t212, t2434);
        let t10073 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk928::<F>(t138, t2438, t785);
        let (t10090, t10102, t10111) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk929::<F>(t555, t9990, t1432, t1433, t9288, t225, t9646);
        let (t10114, t10115) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk930::<F>(t10111, t1428, t22, t2452);
    (t10001, t10022, t10023, t10035, t10069, t10073, t10090, t10102, t10111, t10114, t10115)
}

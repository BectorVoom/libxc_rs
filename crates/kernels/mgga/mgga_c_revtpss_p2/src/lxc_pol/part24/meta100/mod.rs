//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta100 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk579;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk580;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk581;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk582;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta100<F: Float>(t389: F, t1941: F, t268: F, t404: F, t1263: F, t159: F, t635: F, t2304: F) -> (F, F, F, F, F, F, F, F) {
        let (t3335, t3336, t3356, t3357, t3360) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk579::<F>(t389, t1941, t268, t404, t1263, t159);
        let t3361 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk580::<F>(t635);
        let t3362 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk581::<F>(t3361);
        let t3367 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk582::<F>(t2304);
    (t3335, t3336, t3356, t3357, t3360, t3361, t3362, t3367)
}

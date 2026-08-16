//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta161 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk983;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk984;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk985;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta161<F: Float>(t4186: F, t905: F, t904: F, t128: F, t2847: F, t2848: F, t4571: F, t4576: F, t4581: F, t291: F, t1596: F, t914: F) -> (F, F, F, F, F, F) {
        let t4583 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk983::<F>(t4186, t905);
        let (t4584, t4585) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk984::<F>(t4583, t904, t128);
        let (t4587, t4589, t4590) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk985::<F>(t2847, t2848, t4571, t4576, t4581, t4585, t291, t1596, t914);
    (t4583, t4584, t4585, t4587, t4589, t4590)
}

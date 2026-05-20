//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta578 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1928;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1929;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta578<F: Float>(t2689: F, t27239: F, t25277: F, t4458: F, t14685: F, t14756: F, t7021: F, t14760: F, t93015: F, t2723: F, t836: F, t886: F, t1955: F, t27198: F, t2769: F, t25309: F, t2453: F, t27212: F, t1032: F, t4469: F, t867: F, t786: F, t1559: F, t2771: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t99091, t99099, t99102, t99113, t99155) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1928::<F>(t2689, t27239, t25277, t4458, t14685, t14756, t7021, t14760, t93015, t2723, t836, t886);
        let (t99191, t99237, t99257, t99270, t99271, t99272, t99277) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1929::<F>(t1955, t27198, t2769, t25309, t2453, t27212, t1032, t4469, t867, t786, t1559, t2771);
    (t99091, t99099, t99102, t99113, t99155, t99191, t99237, t99257, t99270, t99271, t99272, t99277)
}

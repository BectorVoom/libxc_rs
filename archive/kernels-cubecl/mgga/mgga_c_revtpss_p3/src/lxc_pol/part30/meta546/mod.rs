//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta546 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1985;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta546<F: Float>(t675: F, t886: F, t11006: F, t256: F, t10115: F, t251: F, t2410: F, t11238: F, t196: F, t3800: F, t12625: F, t458: F) -> (F, F, F, F, F, F, F) {
        let (t41040, t41077, t41117, t41154, t42859, t44126, t44841) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1985::<F>(t675, t886, t11006, t256, t10115, t251, t2410, t11238, t196, t3800, t12625, t458);
    (t41040, t41077, t41117, t41154, t42859, t44126, t44841)
}

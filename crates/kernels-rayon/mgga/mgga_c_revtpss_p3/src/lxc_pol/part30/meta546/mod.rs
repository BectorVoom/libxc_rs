//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta546 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1985;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta546(t675: f64, t886: f64, t11006: f64, t256: f64, t10115: f64, t251: f64, t2410: f64, t11238: f64, t196: f64, t3800: f64, t12625: f64, t458: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t41040, t41077, t41117, t41154, t42859, t44126, t44841) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1985(t675, t886, t11006, t256, t10115, t251, t2410, t11238, t196, t3800, t12625, t458);
    (t41040, t41077, t41117, t41154, t42859, t44126, t44841)
}

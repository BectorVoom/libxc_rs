//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta477 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1810;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1811;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1812;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta477<F: Float>(t25460: F, t994: F, t1976: F, t3325: F, t7160: F, t3075: F, t7145: F, t1982: F, t3259: F, t1972: F, t3223: F, t1024: F, t7125: F) -> (F, F, F, F, F, F) {
        let t25476 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1810::<F>(t25460, t994);
        let (t25480, t25484, t25487, t25490) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1811::<F>(t1976, t3325, t7160, t3075, t7145, t1982, t3259, t1972, t3223);
        let t25495 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1812::<F>(t1024, t7125);
    (t25476, t25480, t25484, t25487, t25490, t25495)
}

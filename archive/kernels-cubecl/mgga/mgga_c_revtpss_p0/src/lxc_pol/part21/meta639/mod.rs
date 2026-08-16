//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta639 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2414;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta639<F: Float>(t11054: F, t892: F, t2985: F, t3010: F, t3013: F, t241: F, t281: F, t283: F, t11321: F, t698: F, t2297: F, t2851: F) -> (F, F, F, F, F, F, F, F) {
        let (t41197, t41224, t41235, t41238, t41245, t41246, t41267, t41270) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2414::<F>(t11054, t892, t2985, t3010, t3013, t241, t281, t283, t11321, t698, t2297, t2851);
    (t41197, t41224, t41235, t41238, t41245, t41246, t41267, t41270)
}

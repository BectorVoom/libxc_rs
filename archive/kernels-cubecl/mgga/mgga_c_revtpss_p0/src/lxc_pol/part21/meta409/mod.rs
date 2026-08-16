//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta409 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1876;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta409<F: Float>(t13107: F, t489: F, t1269: F, t3601: F, t3769: F, t1248: F, t1287: F, t3727: F, t3584: F, t3759: F, t11239: F, t1243: F) -> (F, F, F, F, F, F) {
        let (t13108, t13111, t13112, t13118, t13121, t13126) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1876::<F>(t13107, t489, t1269, t3601, t3769, t1248, t1287, t3727, t3584, t3759, t11239, t1243);
    (t13108, t13111, t13112, t13118, t13121, t13126)
}

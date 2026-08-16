//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta499 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1884;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1885;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1886;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta499<F: Float>(t26034: F, t545: F, t2028: F, t3920: F, t7246: F, t2023: F, t2453: F, t3908: F, t2022: F, t3923: F, t543: F, t7301: F, t72: F, t7307: F, t686: F, t7284: F, t1426: F, t786: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t26035, t26036, t26040, t26041, t26043, t26044, t26046) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1884::<F>(t26034, t545, t2028, t3920, t7246, t2023, t2453, t3908, t2022, t3923, t543, t7301);
        let (t26049, t26050) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1885::<F>(t72, t7307, t686);
        let (t26051, t26053, t26054) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1886::<F>(t26050, t7284, t1426, t2023, t786);
    (t26035, t26036, t26040, t26041, t26043, t26044, t26046, t26049, t26050, t26051, t26053, t26054)
}

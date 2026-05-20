//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta496 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1809;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1810;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1811;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta496<F: Float>(t1416: F, t25978: F, t3999: F, t64: F, t239: F, t820: F, t240: F, t7262: F, t3994: F, t2661: F, t2482: F, t27: F) -> (F, F, F, F, F, F, F) {
        let (t25979, t25981) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1809::<F>(t1416, t25978, t3999, t64);
        let (t25983, t25986) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1810::<F>(t239, t25981, t820, t240, t7262);
        let (t25987, t25988, t25997) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1811::<F>(t25986, t3994, t2661, t2482, t27, t7262);
    (t25979, t25981, t25983, t25986, t25987, t25988, t25997)
}

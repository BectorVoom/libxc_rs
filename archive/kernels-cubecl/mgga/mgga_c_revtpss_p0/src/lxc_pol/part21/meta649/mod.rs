//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta649 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2434;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2435;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta649<F: Float>(t367: F, t371: F, t373: F, t9291: F, t1058: F, t11907: F, t3197: F, t3201: F, t11962: F, t3231: F, t11973: F, t11904: F, t11773: F, t11865: F, t11941: F, t11942: F, t127: F, t11937: F, t11947: F, t3205: F, t3206: F, t676: F, t11643: F, t11994: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t42121, t42122, t42124, t42139, t42141, t42146, t42149) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2434::<F>(t367, t371, t373, t9291, t1058, t11907, t3197, t3201, t11962, t3231, t11973, t11904);
        let (t42155, t42170, t42172, t42176, t42190) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2435::<F>(t11773, t11865, t11941, t11942, t127, t371, t11937, t11947, t3205, t3206, t676, t11643, t11994);
    (t42121, t42122, t42124, t42139, t42141, t42146, t42149, t42155, t42170, t42172, t42176, t42190)
}

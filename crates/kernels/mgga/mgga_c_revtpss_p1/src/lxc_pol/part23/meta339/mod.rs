//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta339 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1641;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta339<F: Float>(t14239: F, t4104: F, t2470: F, t5740: F, t4101: F, t1432: F, t5763: F, t1385: F, t5710: F, t1904: F, t3899: F, t689: F) -> (F, F, F, F, F, F, F) {
        let (t14241, t14242, t14243, t14252, t14255, t14274, t14276) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1641::<F>(t14239, t4104, t2470, t5740, t4101, t1432, t5763, t1385, t5710, t1904, t3899, t689);
    (t14241, t14242, t14243, t14252, t14255, t14274, t14276)
}

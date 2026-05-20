//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta359 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1673;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta359<F: Float>(t14987: F, t2467: F, t122: F, t4480: F, t2466: F, t10995: F, t11044: F, t4481: F, t2435: F, t4477: F, t136: F, t1579: F) -> (F, F, F, F, F, F, F) {
        let (t14989, t14990, t14991, t14992, t14995, t14998, t15002) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1673::<F>(t14987, t2467, t122, t4480, t2466, t10995, t11044, t4481, t2435, t4477, t136, t1579);
    (t14989, t14990, t14991, t14992, t14995, t14998, t15002)
}

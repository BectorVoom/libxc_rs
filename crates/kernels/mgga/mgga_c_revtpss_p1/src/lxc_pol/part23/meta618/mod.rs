//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta618 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2295;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta618<F: Float>(t24633: F, t482: F, t371: F, t372: F, t24610: F, t5302: F, t1042: F, t23842: F, t1774: F, t5825: F, t5296: F, t24244: F, t5308: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t24634, t24636, t24639, t24640, t24643, t24644, t24647, t24648, t24649, t24652) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2295::<F>(t24633, t482, t371, t372, t24610, t5302, t1042, t23842, t1774, t5825, t5296, t24244, t5308);
    (t24634, t24636, t24639, t24640, t24643, t24644, t24647, t24648, t24649, t24652)
}

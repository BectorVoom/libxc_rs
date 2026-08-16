//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta749 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2538;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta749<F: Float>(t2873: F, t4587: F, t11298: F, t1596: F, t11466: F, t1633: F, t11299: F, t1609: F, t51913: F, t51915: F, t51973: F, t52035: F) -> (F, F, F, F, F, F, F, F) {
        let (t52505, t52508, t52511, t52514, t52546, t52547, t52573, t52597) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2538::<F>(t2873, t4587, t11298, t1596, t11466, t1633, t11299, t1609, t51913, t51915, t51973, t52035);
    (t52505, t52508, t52511, t52514, t52546, t52547, t52573, t52597)
}

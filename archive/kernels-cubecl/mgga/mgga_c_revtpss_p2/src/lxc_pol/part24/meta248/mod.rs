//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta248 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1011;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta248<F: Float>(t136: F, t1568: F, t2457: F, t2710: F, t2470: F, t4522: F, t874: F, t2718: F, t1569: F, t867: F, t786: F, t2435: F, t4477: F) -> (F, F, F, F, F, F, F) {
        let (t14946, t14948, t14951, t14961, t14986, t14987, t14998) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1011::<F>(t136, t1568, t2457, t2710, t2470, t4522, t874, t2718, t1569, t867, t786, t2435, t4477);
    (t14946, t14948, t14951, t14961, t14986, t14987, t14998)
}

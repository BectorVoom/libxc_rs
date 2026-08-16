//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta515 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1837;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta515<F: Float>(t233: F, t41077: F, t1955: F, t92888: F, t7056: F, t9646: F, t1954: F, t39643: F, t2453: F, t25309: F, t25304: F, t251: F) -> (F, F, F, F, F, F, F, F) {
        let (t93118, t93126, t93134, t93139, t93140, t93157, t93160, t93169) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1837::<F>(t233, t41077, t1955, t92888, t7056, t9646, t1954, t39643, t2453, t25309, t25304, t251);
    (t93118, t93126, t93134, t93139, t93140, t93157, t93160, t93169)
}

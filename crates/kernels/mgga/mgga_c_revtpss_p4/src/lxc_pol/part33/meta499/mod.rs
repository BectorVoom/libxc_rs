//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta499 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1806;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta499<F: Float>(t2122: F, t25163: F, t2139: F, t3655: F, t1256: F, t7610: F, t2138: F, t3670: F, t3666: F, t3678: F, t7613: F, t3685: F, t7607: F) -> (F, F, F, F, F, F, F) {
        let (t26795, t26821, t26822, t26824, t26827, t26832, t26836) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1806::<F>(t2122, t25163, t2139, t3655, t1256, t7610, t2138, t3670, t3666, t3678, t7613, t3685, t7607);
    (t26795, t26821, t26822, t26824, t26827, t26832, t26836)
}

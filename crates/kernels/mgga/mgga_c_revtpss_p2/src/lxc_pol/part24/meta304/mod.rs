//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta304 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1089;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta304<F: Float>(t21451: F, t460: F, t1811: F, t3781: F, t1770: F, t5462: F, t473: F, t6695: F, t5477: F, t20849: F, t487: F, t5812: F, t602: F) -> (F, F, F, F, F, F, F, F) {
        let (t21452, t21455, t21456, t21500, t21541, t21579, t21621, t21663) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1089::<F>(t21451, t460, t1811, t3781, t1770, t5462, t473, t6695, t5477, t20849, t487, t5812, t602);
    (t21452, t21455, t21456, t21500, t21541, t21579, t21621, t21663)
}

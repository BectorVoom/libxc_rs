//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta849 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2989;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta849<F: Float>(t13729: F, t2782: F, t4131: F, t556: F, t47506: F, t5722: F, t1353: F, t198: F, t3829: F, t1868: F, t4135: F, t14304: F, t1450: F) -> (F, F, F, F, F, F) {
        let (t49522, t49528, t49541, t49544, t49582, t49647) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2989::<F>(t13729, t2782, t4131, t556, t47506, t5722, t1353, t198, t3829, t1868, t4135, t14304, t1450);
    (t49522, t49528, t49541, t49544, t49582, t49647)
}

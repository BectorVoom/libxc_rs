//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta490 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2079;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta490<F: Float>(t15525: F, t4733: F, t981: F, t15495: F, t300: F, t15234: F, t964: F, t973: F, t2986: F, t4707: F, t974: F, t11506: F, t1633: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t15526, t15528, t15530, t15534, t15536, t15537, t15538, t15540, t15541) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2079::<F>(t15525, t4733, t981, t15495, t300, t15234, t964, t973, t2986, t4707, t974, t11506, t1633);
    (t15526, t15528, t15530, t15534, t15536, t15537, t15538, t15540, t15541)
}

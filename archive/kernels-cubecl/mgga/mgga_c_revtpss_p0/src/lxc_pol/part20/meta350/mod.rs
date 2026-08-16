//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta350 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1278;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta350<F: Float>(t17879: F, t460: F, t1269: F, t3766: F, t13126: F, t487: F, t1204: F, t5462: F, t3566: F, t488: F, t1209: F, t1045: F, t999: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t17880, t17887, t17888, t17948, t17949, t17955, t17973, t17986, t19620) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1278::<F>(t17879, t460, t1269, t3766, t13126, t487, t1204, t5462, t3566, t488, t1209, t1045, t999);
    (t17880, t17887, t17888, t17948, t17949, t17955, t17973, t17986, t19620)
}

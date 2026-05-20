//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta416 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1361;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1362;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta416<F: Float>(t43043: F, t4891: F, t3057: F, t3298: F, t11773: F, t11926: F, t11858: F, t15688: F, t12077: F, t15905: F, t994: F, t11725: F, t828: F, t225: F, t42059: F, t366: F, t2857: F, t3154: F, t271: F, t2852: F, t41296: F, t11986: F, t11631: F, t905: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t43044, t43050, t43069, t43082, t43105, t43131) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1361::<F>(t43043, t4891, t3057, t3298, t11773, t11926, t11858, t15688, t12077, t15905, t994, t11725, t828);
        let (t43154, t43155, t43174, t43223, t43240, t43253) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1362::<F>(t225, t42059, t366, t2857, t3154, t271, t2852, t41296, t11986, t828, t11631, t905);
    (t43044, t43050, t43069, t43082, t43105, t43131, t43154, t43155, t43174, t43223, t43240, t43253)
}

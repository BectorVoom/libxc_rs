//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta671 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2406;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2407;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta671<F: Float>(t271: F, t2852: F, t1054: F, t11970: F, t11986: F, t828: F, t11631: F, t905: F, t606: F, t1086: F, t11223: F, t3090: F, t11200: F, t11671: F, t11926: F, t16565: F, t994: F, t42859: F, t42862: F, t342: F, t3145: F, t368: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t43222, t43238, t43240, t43254, t43285) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2406::<F>(t271, t2852, t1054, t11970, t11986, t828, t11631, t905, t606, t1086, t11223, t3090);
        let (t43291, t43297, t43341, t43346, t43347, t43350) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2407::<F>(t1086, t11200, t3090, t11671, t11926, t16565, t994, t42859, t42862, t342, t3145, t368);
    (t43222, t43238, t43240, t43254, t43285, t43291, t43297, t43341, t43346, t43347, t43350)
}

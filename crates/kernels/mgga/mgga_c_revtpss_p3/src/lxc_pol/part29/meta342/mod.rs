//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta342 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1262;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1263;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta342<F: Float>(t3111: F, t3188: F, t3211: F, t3215: F, t1026: F, t371: F, t676: F, t1025: F, t271: F, t2857: F, t283: F, t3298: F, t994: F, t4891: F, t1086: F, t3046: F, t3090: F, t3316: F, t1016: F, t697: F, t1011: F, t1010: F, t2270: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t11802, t11814, t11818, t11821, t11852, t11858) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1262::<F>(t3111, t3188, t3211, t3215, t1026, t371, t676, t1025, t271, t2857, t283, t3298, t994);
        let (t11859, t11866, t11875, t11881, t11883) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1263::<F>(t11858, t4891, t1086, t3046, t3090, t3316, t994, t1016, t697, t1011, t1010, t2270);
    (t11802, t11814, t11818, t11821, t11852, t11859, t11866, t11875, t11881, t11883)
}

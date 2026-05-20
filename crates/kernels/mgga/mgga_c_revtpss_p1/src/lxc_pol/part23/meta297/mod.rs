//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta297 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1542;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1543;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1544;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta297<F: Float>(t11788: F, t366: F, t1026: F, t371: F, t676: F, t1025: F, t271: F, t2857: F, t11144: F, t11150: F, t3252: F, t283: F, t66: F, t3298: F, t994: F, t4891: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t11789, t11817, t11818, t11821) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1542::<F>(t11788, t366, t1026, t371, t676, t1025, t271, t2857);
        let (t11822, t11827, t11852) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1543::<F>(t11144, t11821, t11150, t3252, t283, t2857);
        let (t11853, t11858, t11859) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1544::<F>(t11852, t66, t3298, t994, t4891);
    (t11789, t11817, t11818, t11821, t11822, t11827, t11852, t11853, t11858, t11859)
}

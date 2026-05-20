//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta603 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2255;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2256;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta603<F: Float>(t1089: F, t23992: F, t23997: F, t24007: F, t3304: F, t3318: F, t5004: F, t6244: F, t1082: F, t24031: F, t24111: F, t23598: F, t24042: F, t380: F, t6258: F, t1024: F, t11940: F, t12122: F, t12127: F, t1647: F, t16502: F, t16544: F, t16584: F, t1689: F, t1692: F, t19566: F, t23959: F, t3204: F, t3287: F, t3317: F, t342: F, t381: F, t4857: F, t6235: F, t6365: F, t6368: F, t6386: F, t6389: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t24132, t24135, t24138, t24141, t24144, t24147, t24152, t24157) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2255::<F>(t1089, t23992, t23997, t24007, t3304, t3318, t5004, t6244, t1082, t24031, t24111, t23598);
        let (t24162, t24167, t24176) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2256::<F>(t24042, t380, t5004, t6258, t1024, t11940, t12122, t12127, t1647, t16502, t16544, t16584, t1689, t1692, t19566, t23959, t24132, t24135, t24138, t24141, t24144, t24147, t24152, t24157, t3204, t3287, t3317, t342, t381, t4857, t6235, t6365, t6368, t6386, t6389);
    (t24132, t24135, t24138, t24141, t24144, t24147, t24152, t24157, t24162, t24167, t24176)
}

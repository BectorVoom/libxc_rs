//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta607 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2501;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2502;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2503;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2504;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2505;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta607<F: Float>(t3291: F, t6258: F, t1082: F, t19380: F, t6271: F, t73: F, t4976: F, t11249: F, t6305: F, t1043: F, t12050: F, t357: F, t6244: F, t999: F, t6234: F, t993: F, t225: F, t18902: F, t19025: F, t19027: F, t19029: F, t19031: F, t19048: F, t19051: F, t19053: F, t19055: F, t19058: F, t19060: F, t19062: F, t19079: F, t19081: F, t19084: F, t19130: F, t19132: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t19438, t19443, t19446, t19447, t19450) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2501::<F>(t3291, t6258, t1082, t19380, t6271, t73, t4976, t11249, t6305);
        let (t19453, t19456) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2502::<F>(t1043, t12050, t357, t19450, t6244, t999);
        let (t19457, t19462) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2503::<F>(t1082, t19456, t6234, t993);
        let t19463 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2504::<F>(t19462, t225);
        let t19466 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2505::<F>(t18902, t19025, t19027, t19029, t19031, t19048, t19051, t19053, t19055, t19058, t19060, t19062, t19079, t19081, t19084, t19130, t19132);
    (t19438, t19443, t19446, t19447, t19450, t19453, t19456, t19457, t19462, t19463, t19466)
}

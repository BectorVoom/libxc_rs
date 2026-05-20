//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta440 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1855;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1856;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta440<F: Float>(t19049: F, t983: F, t15547: F, t1642: F, t4719: F, t4725: F, t6104: F, t914: F, t936: F, t15416: F, t1610: F, t4590: F, t4632: F, t11134: F, t11534: F, t15127: F, t15189: F, t15503: F, t15504: F, t18906: F, t18911: F, t18915: F, t18919: F, t18924: F, t18928: F, t18932: F, t18934: F, t18939: F, t18944: F, t18948: F) -> (F, F, F, F, F, F, F, F) {
        let (t19051, t19053, t19055, t19056, t19058, t19060, t19062) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1855::<F>(t19049, t983, t15547, t1642, t4719, t4725, t6104, t914, t936, t15416, t1610, t4590, t4632);
        let t19077 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1856::<F>(t11134, t11534, t15127, t15189, t15503, t15504, t18906, t18911, t18915, t18919, t18924, t18928, t18932, t18934, t18939, t18944, t18948);
    (t19051, t19053, t19055, t19056, t19058, t19060, t19062, t19077)
}

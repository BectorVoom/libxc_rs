//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta591 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2472;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2473;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2474;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta591<F: Float>(t18281: F, t905: F, t904: F, t128: F, t11134: F, t11304: F, t15189: F, t15209: F, t15210: F, t15211: F, t18906: F, t18911: F, t18915: F, t18919: F, t18924: F, t18928: F, t18932: F, t18934: F, t18939: F, t18944: F) -> (F, F, F, F) {
        let t18946 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2472::<F>(t18281, t905);
        let (t18947, t18948) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2473::<F>(t18946, t904, t128);
        let t18950 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2474::<F>(t11134, t11304, t15189, t15209, t15210, t15211, t18906, t18911, t18915, t18919, t18924, t18928, t18932, t18934, t18939, t18944, t18948);
    (t18946, t18947, t18948, t18950)
}

//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta604 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1941;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1942;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta604<F: Float>(t5989: F, t92978: F, t18634: F, t27261: F, t18334: F, t25270: F, t25277: F, t5985: F, t18394: F, t7025: F, t27221: F, t62403: F, t18352: F, t1945: F, t807: F, t61639: F, t99062: F, t61725: F, t6017: F, t886: F, t1955: F, t27212: F, t6022: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t106082, t106085, t106088, t106090, t106093, t106099) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1941::<F>(t5989, t92978, t18634, t27261, t18334, t25270, t25277, t5985, t18394, t7025, t27221, t62403);
        let (t106102, t106104, t106106, t106143, t106172, t106228) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1942::<F>(t18352, t1945, t807, t61639, t99062, t27221, t61725, t6017, t886, t1955, t27212, t6022);
    (t106082, t106085, t106088, t106090, t106093, t106099, t106102, t106104, t106106, t106143, t106172, t106228)
}

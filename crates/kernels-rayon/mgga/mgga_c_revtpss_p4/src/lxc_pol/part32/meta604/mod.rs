//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta604 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1941;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1942;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta604(t5989: f64, t92978: f64, t18634: f64, t27261: f64, t18334: f64, t25270: f64, t25277: f64, t5985: f64, t18394: f64, t7025: f64, t27221: f64, t62403: f64, t18352: f64, t1945: f64, t807: f64, t61639: f64, t99062: f64, t61725: f64, t6017: f64, t886: f64, t1955: f64, t27212: f64, t6022: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t106082, t106085, t106088, t106090, t106093, t106099) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1941(t5989, t92978, t18634, t27261, t18334, t25270, t25277, t5985, t18394, t7025, t27221, t62403);
        let (t106102, t106104, t106106, t106143, t106172, t106228) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1942(t18352, t1945, t807, t61639, t99062, t27221, t61725, t6017, t886, t1955, t27212, t6022);
    (t106082, t106085, t106088, t106090, t106093, t106099, t106102, t106104, t106106, t106143, t106172, t106228)
}

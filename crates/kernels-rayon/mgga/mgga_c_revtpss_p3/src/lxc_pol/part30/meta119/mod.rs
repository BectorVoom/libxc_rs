//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta119 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk686;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk687;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta119(t2662: f64, t2664: f64, t2661: f64, t240: f64, t596: f64, t243: f64, t816: f64, t813: f64, t2482: f64, t27: f64, t849: f64, t136: f64, t854: f64, t221: f64, t775: f64, t26: f64, t66: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2665, t2666, t2668, t2670, t2672, t2674) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk686(t2662, t2664, t2661, t240, t596, t243, t816, t813, t2482, t27, t849);
        let (t2675, t2677, t2678, t2681) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk687(t136, t854, t221, t775, t2674, t26, t66);
    (t2665, t2666, t2668, t2670, t2672, t2674, t2675, t2677, t2678, t2681)
}

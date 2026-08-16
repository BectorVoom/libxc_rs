//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta39 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk282;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk283;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk284;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk285;
use chunk4::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk286;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta39(t72: f64, t854: f64, t213: f64, t251: f64, t256: f64, t225: f64, t212: f64, t233: f64, t689: f64, t234: f64, t786: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t855, t865) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk282(t72, t854, t213, t251);
        let (t866, t867, t868) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk283(t256, t225);
        let t869 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk284(t212, t225);
        let (t870, t871, t873, t874) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk285(t233, t251, t869, t689, t234, t786);
        let t875 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk286(t251, t72);
    (t855, t865, t866, t867, t868, t869, t870, t871, t873, t874, t875)
}

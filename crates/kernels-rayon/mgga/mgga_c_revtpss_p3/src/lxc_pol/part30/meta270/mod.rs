//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta270 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1187;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1188;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1189;
use chunk3::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1190;
use chunk4::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1191;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta270(t5: f64, t1923: f64, t2123: f64, t6954: f64, t6960: f64, t6963: f64, t7566: f64, t7576: f64, t7579: f64, t117: f64, t116: f64, t2126: f64, t30: f64, t265: f64, t393: f64, t2163: f64, t670: f64, t7193: f64, t2129: f64, t45: f64, t606: f64, t7099: f64, t1209: f64, t2142: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64, t1219: f64, t2134: f64, t2133: f64, t800: f64, t1230: f64, t2138: f64, t1234: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t7583, t7584) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1187(t5, t1923, t2123, t6954, t6960, t6963, t7566, t7576, t7579, t117);
        let t7586 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1188(t116, t2126);
        let (t7591, t7594, t7599, t7602) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1189(t30, t265, t393, t2163, t670, t7193, t2129, t45, t606, t7099, t1209, t2142, dens_threshold, rho0, zeta_threshold);
        let (t7606, t7607) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1190(t1219, t2134, t2133, t800);
        let (t7610, t7613) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1191(t1230, t2138, t1234);
    (t7583, t7584, t7586, t7591, t7594, t7599, t7602, t7606, t7607, t7610, t7613)
}

//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta92 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk586;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk587;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk588;
use chunk3::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk589;
use chunk4::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk590;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta92(t1963: f64, t30: f64, t1940: f64, t1962: f64, t207: f64, t198: f64, t892: f64, t33: f64, t1312: f64, t1936: f64, t196: f64, t511: f64, t197: f64, t1941: f64, t533: f64, t816: f64, t546: f64, t64: f64, t213: f64, t552: f64, t225: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t1966, t1993, t1995, t2002, t2010, t2013) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk586(t1963, t30, t1940, t1962, t207, t198, t892, t33, t1312, t1936, t196, t511);
        let t2014 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk587(t197, t2013);
        let (t2016, t2018) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk588(t1941, t533, t816, t546, t64);
        let (t2019, t2022) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk589(t2018, t213, t552, t2016);
        let t2023 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk590(t2022, t225);
    (t1966, t1993, t1995, t2002, t2010, t2013, t2014, t2018, t2019, t2022, t2023)
}

//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta94 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;
mod chunk8;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk597;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk598;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk599;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk600;
use chunk4::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk601;
use chunk5::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk602;
use chunk6::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk603;
use chunk7::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk604;
use chunk8::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk605;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta94(t1982: f64, t378: f64, t1032: f64, t1078: f64, t1976: f64, t359: f64, t265: f64, t393: f64, t1978: f64, t342: f64, t1962: f64, t207: f64, t198: f64, t892: f64, t1102: f64, t336: f64, t30: f64, t502: f64, t1966: f64, t45: f64, t1963: f64, t33: f64, t1940: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64, t57: f64, rho1: f64, t1312: f64, t1936: f64, t1932: f64, t196: f64, t511: f64, t197: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t1983 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk597(t1982, t378);
        let t1984 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk598(t1032, t1078);
        let t1985 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk599(t1976, t359);
        let t1986 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk600(t1984, t1985);
        let (t1989, t1993, t1995, t1996) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk601(t265, t393, t1978, t1983, t1986, t342, t1962, t207, t198, t892, t1102, t336);
        let (t1999, t2002, t2003) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk602(t30, t265, t502, t1966, t1996, t45, t1963, t33, t1940, t1995, dens_threshold, rho0, zeta_threshold);
        let t2007 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk603(t33, t2002, t2003, t57, t1999, dens_threshold, rho1, zeta_threshold);
        let t2011 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk604(t1312, t1936, t1932);
        let (t2013, t2014) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk605(t196, t511, t197);
    (t1983, t1984, t1985, t1986, t1989, t1993, t1996, t2003, t2007, t2011, t2013, t2014)
}

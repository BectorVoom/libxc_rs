//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta94 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk573;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk574;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk575;
use chunk3::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk576;
use chunk4::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk577;
use chunk5::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk578;
use chunk6::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk579;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta94(t2002: f64, t544: f64, t559: f64, t1998: f64, t562: f64, t214: f64, t1985: f64, t63: f64, t67: f64, t1864: f64, t5: f64, t1860: f64, t112: f64, t109: f64, t1871: f64, t510: f64, t1888: f64, t1896: f64, t1900: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2003, t2004, t2009, t2010, t2011, t2031) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk573(t2002, t544, t559, t1998, t562, t214, t1985, t63, t67);
        let t2032 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk574(t1864, t2031);
        let t2035 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk575(t5, t1860, t2032);
        let t2036 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk576(t112, t2035);
        let t2039 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk577(t109, t1871);
        let t2040 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk578(t2039, t510);
        let t2047 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk579(t1888, t1896, t1900);
    (t2003, t2004, t2009, t2010, t2011, t2031, t2032, t2035, t2036, t2039, t2040, t2047)
}

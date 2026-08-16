//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta520 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1923;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1924;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1925;
use chunk3::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1926;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta520(t27883: f64, t786: f64, t7286: f64, t1903: f64, t7274: f64, t7296: f64, t25902: f64, t25905: f64, t25914: f64, t25919: f64, t25921: f64, t25941: f64, t25948: f64, t25951: f64, t27885: f64, t27889: f64, t27891: f64, t27896: f64, t7295: f64, t7921: f64, t213: f64, t7910: f64, t5629: f64, t7271: f64, t1885: f64, t26024: f64, t25972: f64, t5622: f64, t1889: f64, t25978: f64, t25986: f64, t5609: f64, t2661: f64, t25973: f64, t25979: f64, t25988: f64, t25998: f64, t26003: f64, t26005: f64, t26011: f64, t26022: f64, t26025: f64, t13846: f64, t1941: f64, t13877: f64, t2018: f64, t5617: f64, t807: f64, t241: f64, t25981: f64, t820: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t27899, t27902, t27903, t27907) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1923(t27883, t786, t7286, t1903, t7274, t7296, t25902, t25905, t25914, t25919, t25921, t25941, t25948, t25951, t27885, t27889, t27891, t27896, t7295, t7921);
        let (t27909, t27919, t27921, t27924, t27926, t27928) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1924(t213, t7910, t5629, t7271, t1885, t26024, t25972, t5622, t1889, t25978, t25986, t5609);
        let t27931 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1925(t2661, t27928, t25973, t25979, t25988, t25998, t26003, t26005, t26011, t26022, t26025, t27919, t27921, t27924, t27926);
        let (t27932, t27933, t27936, t27937, t27940) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1926(t13846, t1941, t13877, t2018, t5617, t807, t241, t25981, t820);
    (t27899, t27902, t27903, t27907, t27909, t27928, t27931, t27932, t27933, t27936, t27937, t27940)
}

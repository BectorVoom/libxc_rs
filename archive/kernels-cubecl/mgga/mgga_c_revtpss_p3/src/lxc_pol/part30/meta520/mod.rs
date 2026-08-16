//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta520 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1923;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1924;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1925;
use chunk3::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1926;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta520<F: Float>(t27883: F, t786: F, t7286: F, t1903: F, t7274: F, t7296: F, t25902: F, t25905: F, t25914: F, t25919: F, t25921: F, t25941: F, t25948: F, t25951: F, t27885: F, t27889: F, t27891: F, t27896: F, t7295: F, t7921: F, t213: F, t7910: F, t5629: F, t7271: F, t1885: F, t26024: F, t25972: F, t5622: F, t1889: F, t25978: F, t25986: F, t5609: F, t2661: F, t25973: F, t25979: F, t25988: F, t25998: F, t26003: F, t26005: F, t26011: F, t26022: F, t26025: F, t13846: F, t1941: F, t13877: F, t2018: F, t5617: F, t807: F, t241: F, t25981: F, t820: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t27899, t27902, t27903, t27907) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1923::<F>(t27883, t786, t7286, t1903, t7274, t7296, t25902, t25905, t25914, t25919, t25921, t25941, t25948, t25951, t27885, t27889, t27891, t27896, t7295, t7921);
        let (t27909, t27919, t27921, t27924, t27926, t27928) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1924::<F>(t213, t7910, t5629, t7271, t1885, t26024, t25972, t5622, t1889, t25978, t25986, t5609);
        let t27931 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1925::<F>(t2661, t27928, t25973, t25979, t25988, t25998, t26003, t26005, t26011, t26022, t26025, t27919, t27921, t27924, t27926);
        let (t27932, t27933, t27936, t27937, t27940) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1926::<F>(t13846, t1941, t13877, t2018, t5617, t807, t241, t25981, t820);
    (t27899, t27902, t27903, t27907, t27909, t27928, t27931, t27932, t27933, t27936, t27937, t27940)
}

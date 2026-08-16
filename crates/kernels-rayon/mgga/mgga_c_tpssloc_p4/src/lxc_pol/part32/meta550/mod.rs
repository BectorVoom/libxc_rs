//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta550 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1905;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1906;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1907;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1908;
use chunk4::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1909;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta550(t1266: f64, t2165: f64, t2167: f64, t2314: f64, t26006: f64, t26141: f64, t26144: f64, t26145: f64, t26147: f64, t26150: f64, t26153: f64, t26157: f64, t4026: f64, t4028: f64, t4034: f64, t5361: f64, t7271: f64, t7983: f64, t7989: f64, t1458: f64, t7408: f64, t2113: f64, t671: f64, t24932: f64, t26109: f64, t26111: f64, t26113: f64, t26116: f64, t26119: f64, t26121: f64, t26123: f64, t26125: f64, t26137: f64, t27371: f64, t27863: f64, t4072: f64, t7266: f64, t1393: f64, t1459: f64, t1849: f64, t26166: f64, t26170: f64, t26178: f64, t26181: f64, t26183: f64, t26505: f64, t4037: f64, t4073: f64, t4077: f64, t574: f64, t652: f64, t7412: f64, t8107: f64, t27860: f64, t27867: f64, t3: f64, t112: f64, t8110: f64, t24969: f64, t24972: f64, t26533: f64, t26535: f64, t26537: f64, t26539: f64, t26541: f64, t26544: f64, t26547: f64, t26549: f64, t26552: f64, t26554: f64, t5376: f64, t577: f64, t7423: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let t27878 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1905(t1266, t2165, t2167, t2314, t26006, t26141, t26144, t26145, t26147, t26150, t26153, t26157, t4026, t4028, t4034, t5361, t7271, t7983, t7989);
        let (t27879, t27888) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1906(t1458, t7408, t2113, t671);
        let t27903 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1907(t1458, t24932, t26109, t26111, t26113, t26116, t26119, t26121, t26123, t26125, t26137, t27371, t27863, t27888, t4072, t671, t7266);
        let t27905 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1908(t1393, t1459, t1849, t24932, t26166, t26170, t26178, t26181, t26183, t26505, t27879, t27888, t27903, t4037, t4073, t4077, t574, t652, t7266, t7412, t8107);
        let (t27907, t27908, t27921, t27930) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1909(t27860, t27867, t27878, t27905, t3, t112, t8110, t1458, t24969, t24972, t26533, t26535, t26537, t26539, t26541, t26544, t26547, t26549, t26552, t26554, t4072, t5376, t577, t671, t7423);
    (t27879, t27888, t27903, t27907, t27908, t27921, t27930)
}

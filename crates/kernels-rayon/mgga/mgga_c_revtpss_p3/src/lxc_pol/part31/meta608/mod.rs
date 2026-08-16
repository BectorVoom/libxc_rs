//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta608 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2047;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2048;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta608(t98206: f64, t2689: f64, t27936: f64, t13857: f64, t94564: f64, t25978: f64, t5629: f64, t1885: f64, t94459: f64, t26024: f64, t5661: f64, t14054: f64, t25986: f64, t2661: f64, t14046: f64, t14050: f64, t13850: f64, t2482: f64, t25981: f64, t814: f64, t13829: f64, t94550: f64, t1873: f64, t94519: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t98207, t98218, t98220, t98222, t98224, t98227, t98229) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2047(t98206, t2689, t27936, t13857, t94564, t25978, t5629, t1885, t94459, t26024, t5661, t14054, t25986, t2661);
        let (t98230, t98236, t98239, t98244, t98259, t98260) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2048(t98229, t14046, t25986, t2661, t14050, t13850, t2482, t25981, t814, t13829, t94550, t1873, t94519);
    (t98207, t98218, t98220, t98222, t98224, t98227, t98230, t98236, t98239, t98244, t98259, t98260)
}

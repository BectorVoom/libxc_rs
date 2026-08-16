//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2054/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2054(t98206: f64, t2689: f64, t27936: f64, t13857: f64, t94564: f64, t25978: f64, t5629: f64, t1885: f64, t94459: f64, t26024: f64, t5661: f64, t14054: f64, t25986: f64, t2661: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t98207 = 0.10164000561857065645e-2_f64 * t98206;
    let t98218 = t2689 * t27936;
    let t98220 = t94564 * t13857;
    let t98222 = t25978 * t5629;
    let t98224 = t94459 * t1885;
    let t98226 = t26024 * t5661;
    let t98227 = 0.40015750243531754508e-2_f64 * t98226;
    let t98229 = t2661 * t25986 * t14054;
    (t98207, t98218, t98220, t98222, t98224, t98227, t98229)
}

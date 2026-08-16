//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1765/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1765(t1131: f64, t1150: f64, t90529: f64, t90542: f64, t90558: f64, t90573: f64, t6439: f64, t68792: f64, t24262: f64, t58342: f64, t12227: f64, t3435: f64, t90324: f64) -> (f64, f64, f64, f64) {
    let t90578 = 1.0_f64 * t1131 * (t90529 + t90542 + t90558 + t90573) * t1150;
    let t90580 = 12.0_f64 * t68792 * t6439;
    let t90582 = 0.3859675079686208416e3_f64 * t58342 * t24262;
    let t90585 = 0.57895126195293126241e3_f64 * t12227 * t90324 * t3435;
    (t90578, t90580, t90582, t90585)
}

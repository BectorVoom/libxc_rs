//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1612/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1612(t13731: f64, t2782: f64, t212: f64, t5710: f64, t1358: f64, t689: f64, t221: f64, t3979: f64, t5591: f64, t3978: f64, t3989: f64, t5614: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t13733 = 0.21951497276451705328e-1_f64 * t2782 * t13731;
    let t13734 = t212 * t5710;
    let t13735 = t13734 * t1358;
    let t13737 = 0.10975748638225852664e-1_f64 * t689 * t13735;
    let t13760 = t3979 * t221 * t5591;
    let t13762 = 0.10164000561857065645e-3_f64 * t3978 * t13760;
    let t13763 = t3989 * t5614;
    (t13733, t13734, t13735, t13737, t13760, t13762, t13763)
}

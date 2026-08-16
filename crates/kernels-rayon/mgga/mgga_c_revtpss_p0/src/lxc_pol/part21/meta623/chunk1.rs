//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2383/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2383(t10732: f64, t10744: f64, t808: f64, t10674: f64, t2710: f64, t2713: f64, t2693: f64, t9732: f64, t14917: f64, t2475: f64, t2661: f64, t2662: f64, t836: f64) -> (f64, f64, f64, f64) {
    let t40529 = t10744 * t808 * t10732;
    let t40532 = t2710 * t2713 * t10674;
    let t40535 = t2710 * t9732 * t2693;
    let t40549 = t2661 * t2662 * t2475 * t836 * t14917;
    (t40529, t40532, t40535, t40549)
}

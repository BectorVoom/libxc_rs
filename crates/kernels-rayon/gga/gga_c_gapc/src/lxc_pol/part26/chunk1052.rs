//! GGA_C_GAPC lxc pol — lxc_pol part 26 (v4rho2sigma2_5) CSE chunk 1052/1308 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part26_v4rho2sigma2_5_chunk1052(t23104: f64, t8676: f64, t10078: f64, t2763: f64, t818: f64, t941: f64, t103: f64, t134: f64, t18679: f64, t15479: f64, t2547: f64, t9933: f64) -> (f64, f64, f64, f64, f64) {
    let t29692 = t8676 * t23104;
    let t29861 = t818 * t2763 * t941 * t10078;
    let t29867 = t134 * t18679 * t103;
    let t29868 = t15479 * t941 * t29867;
    let t30095 = t2547 * t9933;
    (t29692, t29861, t29867, t29868, t30095)
}

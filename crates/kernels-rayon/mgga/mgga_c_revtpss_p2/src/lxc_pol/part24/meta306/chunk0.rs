//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1091/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1091(t5916: f64, t625: f64, t10227: f64, t5895: f64, t10241: f64, t5907: f64, t6785: f64, t9335: f64, t6792: f64, t9350: f64, t1450: f64, t6922: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t21827 = t625 * t5916;
    let t21835 = t10227 * t5895;
    let t21860 = t10241 * t5907;
    let t21906 = t9335 * t6785;
    let t21918 = t9350 * t6792;
    let t21937 = t6922 * t1450;
    (t21827, t21835, t21860, t21906, t21918, t21937)
}

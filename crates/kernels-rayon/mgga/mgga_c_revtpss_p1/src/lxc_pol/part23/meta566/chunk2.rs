//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2145/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2145(t1868: f64, t4003: f64, t22046: f64, t3936: f64, t124: f64, t22809: f64, t800: f64, t6816: f64) -> (f64, f64, f64, f64, f64) {
    let t22841 = t4003 * t1868;
    let t22843 = t3936 * t22046 * t22841;
    let t22848 = t124 * t22809;
    let t22849 = t800 * t22848;
    let t22852 = t6816 * t1868;
    (t22841, t22843, t22848, t22849, t22852)
}

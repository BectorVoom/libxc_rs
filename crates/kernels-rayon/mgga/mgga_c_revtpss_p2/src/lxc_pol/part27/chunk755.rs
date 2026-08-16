//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 755/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk755(t1294: f64, t2142: f64, t7652: f64, t3140: f64, t487: f64, t1276: f64, t2148: f64) -> (f64, f64, f64, f64) {
    let t7653 = t2142 * t1294;
    let t7654 = t7652 * t7653;
    let t7657 = t487 * t3140;
    let t7658 = t7657 * t1276;
    let t7659 = t2148 * t7658;
    (t7653, t7654, t7658, t7659)
}

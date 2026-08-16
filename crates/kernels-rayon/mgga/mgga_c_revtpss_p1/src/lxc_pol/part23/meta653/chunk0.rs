//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2381/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2381(t2677: f64, t40710: f64, t234: f64, t9801: f64, t10887: f64, t136: f64, t2475: f64, t220: f64, t2482: f64, t2668: f64, t823: f64, t159: f64, t33127: f64, t64: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t40711 = t40710 * t2677;
    let t40721 = t9801 * t234;
    let t40722 = t40721 * t10887;
    let t40724 = t2475 * t136;
    let t40725 = t40724 * t220;
    let t40731 = t2482 * t823 * t2668;
    let t40735 = t64 * t33127 * t159;
    (t40711, t40721, t40722, t40724, t40725, t40731, t40735)
}

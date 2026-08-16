//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 1885/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1885(t300: f64, t3488: f64, t3800: f64, t498: f64, t1204: f64, t1269: f64, t12295: f64, t1207: f64, t456: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t12571 = t300 * t3488;
    let t12587 = 1.0_f64 / t3800 / t498;
    let t12603 = t1204 * t1269;
    let t12610 = 0.46096296296296296297e-1_f64 * t12295;
    let t12625 = t1207 * t1207;
    let t12626 = 1.0_f64 / t12625;
    let t12627 = t456 * t12626;
    (t12571, t12587, t12603, t12610, t12625, t12626, t12627)
}

//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2170/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2170(t14336: f64, t14339: f64, t1544: f64, t18860: f64, t5966: f64) -> (f64, f64, f64, f64) {
    let t23106 = 0.51947577317044391276e2_f64 * t14336;
    let t23110 = 0.73245789224026180216e-3_f64 * t14339;
    let t23111 = t18860 * t1544;
    let t23114 = t5966 * t1544;
    (t23106, t23110, t23111, t23114)
}

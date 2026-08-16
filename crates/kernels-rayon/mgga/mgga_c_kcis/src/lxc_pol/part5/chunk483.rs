//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 483/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk483(t1995: f64, t542: f64, t1102: f64, t1470: f64, t1924: f64, t1988: f64, t1992: f64, t344: f64, t486: f64) -> (f64, f64) {
    let t1996 = t542 * t1995;
    let t2001 = t1470 + 0.65704296666666666667e-3_f64 * t1102 * t1988 + 0.1478346675e-2_f64 * t344 * t1992 - 0.98556445e-3_f64 * t344 * t1996 - 4.0_f64 * t486 * t1924;
    (t1996, t2001)
}

//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 910/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk910(t214: f64, t33428: f64, t1880: f64, t1484: f64, t31337: f64, t23270: f64, t22986: f64, t1527: f64, t31332: f64, t1888: f64, t32212: f64, t33159: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t33429 = t214 * t33428;
    let t33430 = t1880 * t33429;
    let t33447 = t31337 * t1484;
    let t33448 = t23270 * t33447;
    let t33449 = t22986 * t33448;
    let t33457 = t31332 * t1527;
    let t33458 = t23270 * t33457;
    let t33459 = t1888 * t33458;
    let t33790 = t32212 * t33159;
    (t33429, t33430, t33447, t33448, t33449, t33457, t33458, t33459, t33790)
}

//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 882/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk882(t1527: f64, t8562: f64, t2718: f64, t1484: f64, t31337: f64, t23270: f64, t22986: f64, t2053: f64, t7537: f64, t31332: f64, t1888: f64, t1528: f64, t30748: f64, t31407: f64, t31423: f64, t31426: f64, t32877: f64, t6627: f64, t7087: f64, t7517: f64, t7830: f64, t855: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t33442 = t8562 * t1527;
    let t33443 = t2718 * t33442;
    let t33447 = t31337 * t1484;
    let t33448 = t23270 * t33447;
    let t33449 = t22986 * t33448;
    let t33452 = t2718 * t2053 * t7537;
    let t33457 = t31332 * t1527;
    let t33458 = t23270 * t33457;
    let t33459 = t1888 * t33458;
    let t33463 = 2.0_f64 * t855 * t33443 + t31407 - t31423 * t1528 + 0.16449340668482264365e-1_f64 * t33449 + 2.0_f64 * t855 * t33452 + 2.0_f64 * t6627 * t7830 + 0.16449340668482264365e-1_f64 * t33459 - t32877 + 2.0_f64 * t7087 * t7517 + t30748 + t31426;
    (t33443, t33447, t33448, t33452, t33457, t33458, t33463)
}

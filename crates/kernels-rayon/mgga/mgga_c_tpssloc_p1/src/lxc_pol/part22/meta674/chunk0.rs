//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2231/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2231(t13822: f64, t17777: f64, t973: f64, t2986: f64, t4514: f64, t48019: f64, t48046: f64, t10236: f64, t17691: f64, t13779: f64, t17183: f64, t16558: f64, t2989: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t61472 = t973 * t13822 * t17777;
    let t61489 = t2986 * t48019 * t4514;
    let t61495 = t2986 * t48046 * t4514;
    let t61528 = t10236 * t17691;
    let t61557 = t2986 * t13779 * t17183;
    let t61589 = t2989 * t16558;
    (t61472, t61489, t61495, t61528, t61557, t61589)
}

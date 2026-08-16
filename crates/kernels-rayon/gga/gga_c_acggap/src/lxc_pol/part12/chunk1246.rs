//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1246/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1246(t2226: f64, t33802: f64, t2131: f64, t2132: f64, t2385: f64, t847: f64, t2230: f64, t33429: f64, t8100: f64, t8397: f64, t8061: f64, t8998: f64) -> (f64, f64, f64, f64, f64) {
    let t38471 = 0.17347256376410398924e1_f64 * t33802 * t2226;
    let t38474 = t2131 * t2132 * t2385 * t847;
    let t38481 = 0.17347256376410398924e1_f64 * t33429 * t2230;
    let t38487 = 0.17347256376410398924e1_f64 * t8397 * t8100;
    let t38489 = 0.17347256376410398924e1_f64 * t8998 * t8061;
    (t38471, t38474, t38481, t38487, t38489)
}

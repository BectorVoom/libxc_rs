//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1264/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1264(t32357: f64, t4820: f64, t7513: f64, t32436: f64, t2679: f64, t8802: f64, t9800: f64, t10942: f64, t1966: f64, t11119: f64, t1986: f64, t1445: f64, t2087: f64, t2530: f64, t8483: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t33483 = 0.15889106645266856297e0_f64 * t7513 * t4820 * t32357;
    let t33486 = 0.15889106645266856297e0_f64 * t7513 * t4820 * t32436;
    let t33492 = t9800 * t8802 * t2679;
    let t33493 = 0.19171462976960374838e1_f64 * t33492;
    let t33494 = t1966 * t10942;
    let t33495 = 0.25561950635947166451e1_f64 * t33494;
    let t33496 = t1986 * t11119;
    let t33497 = 0.51123901271894332902e0_f64 * t33496;
    let t33501 = 0.13803453343411469884e2_f64 * t2087 * t1445 * t8483 * t2530;
    (t33483, t33486, t33493, t33495, t33497, t33501)
}

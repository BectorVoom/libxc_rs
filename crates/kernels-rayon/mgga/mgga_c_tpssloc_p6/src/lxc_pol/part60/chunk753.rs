//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 753/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk753(t1222: f64, t8049: f64, t5017: f64, t7337: f64, t1207: f64, t2139: f64, t5022: f64, t471: f64, t1714: f64, t52: f64, t2132: f64, t24746: f64) -> (f64, f64, f64, f64, f64) {
    let t27592 = t8049 * t1222;
    let t27598 = t7337 * t5017;
    let t27599 = t1207 * t27598;
    let t27603 = t2139 * t5022;
    let t27604 = t471 * t27603;
    let t27607 = t52 * t1714;
    let t27608 = t2132 * t27607;
    let t27609 = t27608 * t24746;
    (t27592, t27599, t27604, t27607, t27609)
}

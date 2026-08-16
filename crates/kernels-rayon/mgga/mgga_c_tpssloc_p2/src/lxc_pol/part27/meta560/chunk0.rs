//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2003/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2003(t13170: f64, t252: f64, t1519: f64, t2678: f64, t13068: f64, t225: f64, t13030: f64, t13062: f64, t13378: f64, t193: f64, t2379: f64, t14538: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t47448 = t252 * t13170;
    let t47528 = t1519 * t2678;
    let t47568 = t13068 * t225;
    let t47585 = t13030 * t225;
    let t47609 = t13062 * t225;
    let t47618 = t13378 * t225;
    let t47645 = t193 * t2379;
    let t50622 = t14538 * t225;
    (t47448, t47528, t47568, t47585, t47609, t47618, t47645, t50622)
}

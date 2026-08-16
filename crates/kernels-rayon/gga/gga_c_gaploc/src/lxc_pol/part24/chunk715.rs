//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 715/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk715(t2335: f64, t4614: f64, t188: f64, t6447: f64, t1564: f64, t2293: f64, t475: f64, t1445: f64, t2304: f64, t524: f64, t6417: f64, t6429: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t6637 = t4614 * t2335;
    let t6642 = t188 * t6447;
    let t6647 = t1564 * t2293;
    let t6648 = t6647 * t475;
    let t6649 = t1445 * t6648;
    let t6652 = t524 * t2304;
    let t6655 = t6417 * t475;
    let t6656 = t1445 * t6655;
    let t6659 = t1445 * t6429;
    (t6637, t6642, t6649, t6652, t6655, t6656, t6659)
}

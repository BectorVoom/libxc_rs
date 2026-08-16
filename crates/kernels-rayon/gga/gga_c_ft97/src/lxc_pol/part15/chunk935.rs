//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 935/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk935(t1775: f64, t20369: f64, t20384: f64, t20356: f64, t20359: f64, t20372: f64, t20366: f64, t20381: f64, t20098: f64, t358: f64, t2: f64, t20337: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t73497 = t1775 * t20369;
    let t73504 = t1775 * t20384;
    let t73506 = t1775 * t20356;
    let t73508 = t1775 * t20359;
    let t73574 = t1775 * t20372;
    let t73576 = t1775 * t20366;
    let t73637 = t1775 * t20381;
    let t73639 = t20098 * t358;
    let t73653 = t2 * t20098;
    let t73662 = t1775 * t20337;
    (t73497, t73504, t73506, t73508, t73574, t73576, t73637, t73639, t73653, t73662)
}

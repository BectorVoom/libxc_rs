//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 3 (v3rho3_1) CSE chunk 1113/1255 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part3_v3rho3_1_chunk1113(t225: f64, t4553: f64, t1634: f64, t3206: f64, t3174: f64, t4559: f64, t4555: f64, t4657: f64, t990: f64, t14488: f64, t381: f64, t1060: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t14545 = t4553 * t225;
    let t14548 = t1634 * t3206;
    let t14549 = t3174 * t14548;
    let t14552 = t4559 * t225;
    let t14555 = t4555 * t225;
    let t14562 = t990 * t4657;
    let t14571 = t381 * t14488;
    let t14572 = t14571 * t1060;
    (t14545, t14549, t14552, t14555, t14562, t14572)
}

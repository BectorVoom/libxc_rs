//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1168/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1168(t1419: f64, t55: f64, t1240: f64, t1760: f64, t2122: f64, t24574: f64, t8003: f64, t6686: f64, t8020: f64) -> (f64, f64, f64, f64, f64) {
    let t27356 = t1419 * t55;
    let t27381 = t1240 * t1760;
    let t27382 = t2122 * t27381;
    let t27401 = t24574 * t8003;
    let t27406 = t8020 * t6686;
    (t27356, t27381, t27382, t27401, t27406)
}

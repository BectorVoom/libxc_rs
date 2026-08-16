//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1105/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1105(t22690: f64, t6968: f64, t22642: f64, t268: f64, t534: f64, t6559: f64) -> (f64, f64, f64) {
    let t22691 = t22690 * t6968;
    let t22692 = t22642 * t22691;
    let t22693 = 0.82246703342411321824e-2_f64 * t22692;
    let t22704 = t6559 * t534 * t268;
    (t22691, t22693, t22704)
}

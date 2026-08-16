//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1172/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1172(t11789: f64, t820: f64, t204: f64, t486: f64, t11716: f64, t44833: f64, t44834: f64, t3503: f64, t3584: f64, t676: f64, t221: f64, t44483: f64, t456: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t44951 = t820 * t11789;
    let t45017 = t204 * t486;
    let t45030 = t44833 * t11716 * t44834;
    let t45037 = t44833 * t3503 * t44834;
    let t45046 = t676 * t3584;
    let t45112 = 5.0_f64 / 486.0_f64 * t456 * t221 * t44483;
    (t44951, t45017, t45030, t45037, t45046, t45112)
}

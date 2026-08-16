//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 475/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk475(t2535: f64, t761: f64, t718: f64, t751: f64, t15: f64, t60: f64, t59: f64, t207: f64, t215: f64, t782: f64, t786: f64, t789: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t2537 = 0.5848223622634646207e0_f64 * t761 * t2535;
    let t2538 = t718 * t751;
    let t2558 = 1.0_f64 / t60 / t15;
    let t2559 = t59 * t2558;
    let t2562 = 0.64814814814814814813e-2_f64 * t2559 * t207 * t215;
    let t2563 = t782 * t786;
    let t2564 = t2563 * t789;
    (t2537, t2538, t2558, t2559, t2562, t2563, t2564)
}

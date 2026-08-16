//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2073/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2073(t241: f64, t6597: f64, t248: f64, t555: f64, t557: f64, t12434: f64, t1338: f64, t12019: f64, t566: f64, t68: f64, t3700: f64, t10121: f64, t870: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t40445 = t6597 * t241;
    let t40449 = 13685.0_f64 / 31104.0_f64 * t555 * t40445 * t557 * t248;
    let t40479 = t1338 * t12434;
    let t40590 = 1.0_f64 / t12019 / t566;
    let t40591 = t68 * t40590;
    let t40610 = t3700 * t3700;
    let t40611 = 1.0_f64 / t40610;
    let t40622 = t10121 * t870;
    (t40445, t40449, t40479, t40591, t40611, t40622)
}

//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2067/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2067(t12250: f64, t3850: f64, t10021: f64, t154: f64, t59: f64, t3749: f64, t598: f64, t535: f64, t795: f64, t215: f64, t39933: f64, t12227: f64, t9577: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t40335 = t12250 * t3850;
    let t40341 = t59 * t10021 * t154;
    let t40343 = 0.99537037037037037035e-1_f64 * t40341 * t3749;
    let t40344 = t59 * t598;
    let t40347 = 0.11265432098765432099e0_f64 * t40344 * t535 * t795;
    let t40350 = 0.14979423868312757201e0_f64 * t39933 * t535 * t215;
    let t40351 = t9577 * t12227;
    (t40335, t40341, t40343, t40344, t40347, t40350, t40351)
}

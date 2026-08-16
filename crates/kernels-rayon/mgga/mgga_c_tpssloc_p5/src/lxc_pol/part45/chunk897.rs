//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 897/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk897(t31668: f64, t533: f64, t1390: f64, t1983: f64, t8511: f64, t9231: f64, t9239: f64, t645: f64, t8513: f64, t8514: f64, t131: f64, t7025: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t31669 = t533 * t31668;
    let t31670 = t31669 * t1390;
    let t31671 = t1983 * t31670;
    let t31672 = t9231 * t8511;
    let t31675 = t9239 * t8511;
    let t31677 = t8513 * t8514 * t645;
    let t31680 = t7025 * t131;
    (t31669, t31670, t31671, t31672, t31675, t31677, t31680)
}

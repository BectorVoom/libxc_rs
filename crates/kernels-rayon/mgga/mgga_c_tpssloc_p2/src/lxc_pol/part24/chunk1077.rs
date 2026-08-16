//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1077/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1077(t12407: f64, t3805: f64, t3806: f64, t12402: f64, t1352: f64, t5248: f64, t1995: f64, t67: f64, t246: f64, t3734: f64, t550: f64, t12368: f64, t3807: f64) -> (f64, f64, f64, f64) {
    let t12409 = t3805 * t3806 * t12407;
    let t12413 = t5248 * t12402 * t1352;
    let t12418 = t1995 * t67;
    let t12419 = t12418 * t246;
    let t12420 = t550 * t3734;
    let t12422 = t12419 * t3806 * t12420;
    let t12426 = t3805 * t12368 * t3807;
    (t12409, t12413, t12422, t12426)
}

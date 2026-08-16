//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1698/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1698(t12680: f64, t607: f64, t2250: f64, t3981: f64, t12606: f64, t43: f64, t1409: f64, t2244: f64, t9300: f64, t2274: f64, t3966: f64, t3990: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t12681 = t12680 * t607;
    let t12684 = t3981 * t2250;
    let t12687 = t43 * t12606;
    let t12695 = t9300 * t1409 * t2244;
    let t12698 = t2274 * t3966;
    let t12699 = t12698 * t607;
    let t12702 = t3990 * t2250;
    (t12681, t12684, t12687, t12695, t12698, t12699, t12702)
}

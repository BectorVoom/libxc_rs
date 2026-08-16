//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1103/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1103(t22641: f64, t9523: f64, t1887: f64, t23069: f64, t6561: f64, t80741: f64, t6643: f64, t2588: f64, t225: f64, t814: f64, t22715: f64, t6551: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t81573 = t22641 * t9523;
    let t81591 = t23069 * t1887;
    let t81597 = t80741 * t6561;
    let t81598 = t81597 * t6643;
    let t81612 = t22641 * t2588;
    let t81613 = t225 * t814;
    let t81632 = t22715 * t6551;
    (t81573, t81591, t81597, t81598, t81612, t81613, t81632)
}

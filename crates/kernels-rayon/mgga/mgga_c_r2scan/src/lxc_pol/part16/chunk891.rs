//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 891/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk891(t1567: f64, t3052: f64, t2124: f64, t2591: f64, t8778: f64, t360: f64, t3055: f64, t6359: f64, t494: f64, t6363: f64, t9317: f64, t8820: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9524 = t1567 * t3052;
    let t9526 = t2124 * t9524 * t2591;
    let t9529 = t8778 * t2591;
    let t9530 = t360 * t9529;
    let t9533 = t6359 * t3055;
    let t9534 = t6363 * t494;
    let t9536 = t2124 * t9533 * t9534;
    let t9540 = t2124 * t9317 * t2591;
    let t9543 = t8820 * t9534;
    (t9526, t9529, t9530, t9536, t9540, t9543)
}
